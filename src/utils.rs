use crate::{IntSpan, Range};
use anyhow::anyhow;
use flate2;
use path_clean::PathClean;
use serde_json::Value;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, io};

/// ```
/// use std::io::BufRead;
/// let reader = intspan::reader("tests/spanr/S288c.chr.sizes");
/// let mut lines = vec![];
/// for line in reader.lines() {
///     lines.push(line);
/// }
/// assert_eq!(lines.len(), 16);
///
/// let reader = intspan::reader("tests/spanr/S288c.chr.sizes");
/// assert_eq!(reader.lines().collect::<Vec<_>>().len(), 16);
/// ```
pub fn reader(input: &str) -> Box<dyn BufRead> {
    let reader: Box<dyn BufRead> = if input == "stdin" {
        Box::new(BufReader::new(io::stdin()))
    } else {
        let path = Path::new(input);
        let file = match File::open(path) {
            Err(why) => panic!("could not open {}: {}", path.display(), why),
            Ok(file) => file,
        };

        if path.extension() == Some(OsStr::new("gz")) {
            Box::new(BufReader::new(flate2::read::MultiGzDecoder::new(file)))
        } else {
            Box::new(BufReader::new(file))
        }
    };

    reader
}

/// ```
/// let lines = intspan::read_lines("tests/spanr/S288c.chr.sizes");
/// assert_eq!(lines.len(), 16);
/// ```
pub fn read_lines(input: &str) -> Vec<String> {
    let mut reader = reader(input);
    let mut s = String::new();
    reader.read_to_string(&mut s).expect("Read error");
    s.lines().map(|s| s.to_string()).collect::<Vec<String>>()
}

/// ```
/// let sizes = intspan::read_sizes("tests/spanr/S288c.chr.sizes");
/// assert_eq!(sizes.len(), 16);
/// assert_eq!(*sizes.get("II").unwrap(), 813184);
/// ```
pub fn read_sizes(input: &str) -> BTreeMap<String, i32> {
    let mut sizes: BTreeMap<String, i32> = BTreeMap::new();

    for line in read_lines(input) {
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() == 2 {
            sizes.insert(fields[0].to_string(), fields[1].parse::<i32>().unwrap());
        }
    }

    sizes
}

/// ```
/// let chrs = intspan::read_first_column("tests/spanr/S288c.chr.sizes");
/// assert_eq!(chrs.len(), 16);
/// assert_eq!(*chrs.get(1).unwrap(), "II");
/// assert_eq!(*chrs.get(15).unwrap(), "XVI");
/// ```
pub fn read_first_column(input: &str) -> Vec<String> {
    let reader = reader(input);
    let mut rows: Vec<String> = Vec::new();

    for line in reader.lines() {
        let field = line.unwrap().split('\t').next().unwrap().to_string();
        rows.push(field);
    }

    rows
}

/// ```
/// let replaces = intspan::read_replaces("tests/spanr/S288c.chr.sizes");
/// assert_eq!(replaces.len(), 16);
/// assert_eq!(*replaces.get("II").unwrap().get(0).unwrap(), "813184");
/// ```
pub fn read_replaces(input: &str) -> BTreeMap<String, Vec<String>> {
    let mut replaces: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for line in read_lines(input) {
        let mut fields: Vec<&str> = line.split('\t').collect();

        let left = fields.split_off(1);

        replaces.insert(
            fields[0].to_string(),
            left.iter().map(|s| (*s).to_string()).collect(),
        );
    }

    replaces
}

pub fn read_json(input: &str) -> BTreeMap<String, Value> {
    let mut reader = reader(input);
    let mut s = String::new();
    reader.read_to_string(&mut s).expect("Read error");

    serde_json::from_str(&s).unwrap()
}

pub fn writer(output: &str) -> Box<dyn Write> {
    let writer: Box<dyn Write> = if output == "stdout" {
        Box::new(BufWriter::new(io::stdout()))
    } else {
        Box::new(BufWriter::new(File::create(output).unwrap()))
    };

    writer
}

pub fn write_lines(output: &str, lines: &Vec<&str>) -> Result<(), std::io::Error> {
    let mut writer = writer(output);

    for line in lines {
        writer.write_all(format!("{}\n", line).as_ref())?;
    }

    Ok(())
}

pub fn write_json(output: &str, json: &BTreeMap<String, Value>) -> Result<(), std::io::Error> {
    let mut writer = writer(output);
    let mut s = serde_json::to_string_pretty(json).unwrap();
    s.push('\n');
    writer.write_all(s.as_bytes())?;

    Ok(())
}

/// ```
/// use serde_json::Value;
/// use std::collections::BTreeMap;
/// let value: Value = serde_json::to_value("28547-29194").unwrap();
/// let mut runlists: BTreeMap<String, Value> = BTreeMap::new();
/// runlists.insert("I".to_string(), value);
///
/// let sets = intspan::json2set(&runlists);
/// assert!(sets.values().next().unwrap().contains(28550));
/// ```
pub fn json2set(json: &BTreeMap<String, Value>) -> BTreeMap<String, IntSpan> {
    let mut set: BTreeMap<String, IntSpan> = BTreeMap::new();

    for (chr, value) in json {
        let intspan = IntSpan::from(value.as_str().unwrap());
        set.insert(chr.into(), intspan);
    }

    set
}

/// ```
/// use serde_json::Value;
/// use std::collections::BTreeMap;
/// use intspan::IntSpan;
/// let mut intspan = IntSpan::new();
/// intspan.add_pair(28547, 29194);
/// let mut set_of: BTreeMap<String, IntSpan> = BTreeMap::new();
/// set_of.insert("I".to_string(), intspan);
///
/// let runlist_of = intspan::set2json(&set_of);
/// assert_eq!(
///     runlist_of.values().next().unwrap(),
///     &Value::String("28547-29194".into())
/// );
/// ```
pub fn set2json(set: &BTreeMap<String, IntSpan>) -> BTreeMap<String, Value> {
    let mut json: BTreeMap<String, Value> = BTreeMap::new();

    for (chr, value) in set {
        let runlist = value.to_string();
        json.insert(chr.into(), serde_json::to_value(runlist).unwrap());
    }

    json
}

pub fn set2json_m(set_of: &BTreeMap<String, BTreeMap<String, IntSpan>>) -> BTreeMap<String, Value> {
    let mut out_json: BTreeMap<String, Value> = BTreeMap::new();

    for (name, set) in set_of {
        let json = set2json(set);
        out_json.insert(name.to_string(), serde_json::to_value(json).unwrap());
    }

    out_json
}

pub fn json2set_m(json: &BTreeMap<String, Value>) -> BTreeMap<String, BTreeMap<String, IntSpan>> {
    let is_multi: bool = json.values().next().unwrap().is_object();

    let mut s_of: BTreeMap<String, BTreeMap<String, IntSpan>> = BTreeMap::new();
    if is_multi {
        for (key, value) in json {
            let string = serde_json::to_string(value).unwrap();
            let runlist_one: BTreeMap<String, Value> =
                serde_json::from_str(string.as_str()).unwrap();
            let set_one = json2set(&runlist_one);
            s_of.insert(key.to_string(), set_one);
        }
    } else {
        let set_one = json2set(json);
        s_of.insert("__single".to_string(), set_one);
    }

    s_of
}

pub fn fill_up_m(
    set_of: &mut BTreeMap<String, BTreeMap<String, IntSpan>>,
    chrs: &BTreeSet<String>,
) {
    for set in set_of.values_mut() {
        for chr in chrs {
            if !set.contains_key(chr) {
                set.insert(chr.into(), IntSpan::new());
            }
        }
    }
}

pub fn fill_up_s(set: &mut BTreeMap<String, IntSpan>, chrs: &BTreeSet<String>) {
    for chr in chrs {
        if !set.contains_key(chr) {
            set.insert(chr.into(), IntSpan::new());
        }
    }
}

pub fn chrs_in_sets(set_of: &BTreeMap<String, BTreeMap<String, IntSpan>>) -> BTreeSet<String> {
    let mut chrs: BTreeSet<String> = BTreeSet::new();

    for name in set_of.keys() {
        for chr in set_of.get(name).unwrap().keys() {
            chrs.insert(chr.clone());
        }
    }

    chrs
}

pub fn build_range_of_part(line: &str, range_of_str: &mut HashMap<String, Range>) {
    for part in line.split('\t') {
        let range = Range::from_str(part);
        if !range.is_valid() {
            continue;
        }

        if !range_of_str.contains_key(part) {
            range_of_str.insert(part.to_string(), range);
        }
    }
}

pub fn sort_links(lines: &[String]) -> Vec<String> {
    // cache ranges
    let mut range_of_part: HashMap<String, Range> = HashMap::new();

    //----------------------------
    // Sort within links
    //----------------------------
    let mut within_links: BTreeSet<String> = BTreeSet::new();
    for line in lines {
        build_range_of_part(line, &mut range_of_part);

        let parts: Vec<&str> = line.split('\t').collect();

        let mut valids: Vec<&str> = parts
            .clone()
            .into_iter()
            .filter(|p| range_of_part.contains_key(*p))
            .collect();

        let mut invalids: Vec<&str> = parts
            .clone()
            .into_iter()
            .filter(|p| !range_of_part.contains_key(*p))
            .collect();

        // by chromosome strand
        valids.sort_by_key(|k| range_of_part.get(*k).unwrap().strand());

        // by start point on chromosomes
        valids.sort_by_key(|k| range_of_part.get(*k).unwrap().start());

        // by chromosome name
        valids.sort_by_key(|k| range_of_part.get(*k).unwrap().chr());

        // recreate line
        valids.append(&mut invalids);
        let new_line: String = valids.join("\t");
        within_links.insert(new_line);
    }

    //----------------------------
    // Sort by first range's chromosome order among links
    //----------------------------
    let mut among_links: Vec<String> = within_links.into_iter().collect();
    {
        // by chromosome strand
        among_links.sort_by_cached_key(|k| {
            let parts: Vec<&str> = k.split('\t').collect();
            range_of_part.get(parts[0]).unwrap().strand()
        });

        // by start point on chromosomes
        among_links.sort_by_cached_key(|k| {
            let parts: Vec<&str> = k.split('\t').collect();
            range_of_part.get(parts[0]).unwrap().start()
        });

        // by chromosome name
        among_links.sort_by_cached_key(|k| {
            let parts: Vec<&str> = k.split('\t').collect();
            range_of_part.get(parts[0]).unwrap().chr()
        });
    }

    //----------------------------
    // Sort by copy number among links (desc)
    //----------------------------
    {
        among_links.sort_by_cached_key(|k| Reverse(k.split('\t').count()));
    }

    among_links
}

/// ```
/// match which::which("samtools") {
///     Ok(_) => {
///         let seq = intspan::get_seq_faidx("tests/fasr/NC_000932.fa", "NC_000932:1-10").unwrap();
///         assert_eq!(seq, "ATGGGCGAAC".to_string());
///         let res = intspan::get_seq_faidx("tests/fasr/NC_000932.fa", "FAKE:1-10");
///         assert_eq!(format!("{}", res.unwrap_err()), "Command executed with failing error code");
///     }
///     Err(_) => {}
/// }
/// ```
// cargo test --doc utils::get_seq_faidx
pub fn get_seq_faidx(file: &str, range: &str) -> anyhow::Result<String> {
    let mut bin = String::new();
    for e in &["samtools"] {
        if let Ok(pth) = which::which(e) {
            bin = pth.to_string_lossy().to_string();
            break;
        }
    }

    if bin.is_empty() {
        return Err(anyhow!("Can't find the external command"));
    }

    let mut seq = String::new();
    let output = Command::new(bin)
        .arg("faidx")
        .arg(file)
        .arg(range)
        .output()?;

    if !output.status.success() {
        return Err(anyhow!("Command executed with failing error code"));
    }

    for line in output.stdout.lines().map_while(Result::ok) {
        // header
        if line.starts_with('>') {
            continue;
        }

        seq += line.as_str();
    }

    Ok(seq)
}

pub fn basename(path: impl AsRef<Path>) -> io::Result<String> {
    let path = path.as_ref();

    let basename = path
        .file_stem()
        .and_then(OsStr::to_str)
        .unwrap()
        .split('.')
        .next()
        .unwrap()
        .to_string();

    Ok(basename)
}

pub fn absolute_path(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    let path = path.as_ref();

    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()?.join(path)
    }
    .clean();

    Ok(absolute_path)
}

#[cfg(test)]
mod read_write {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_write_lines() {
        let tmp = TempDir::new().unwrap();
        let filename = tmp
            .path()
            .join("test.txt")
            .into_os_string()
            .into_string()
            .unwrap();
        write_lines(&filename, &vec!["This", "is", "a\ntest"]).expect("Write error");

        let lines = read_lines(&filename);
        assert_eq!(lines.len(), 4);
    }

    #[test]
    fn test_read_write_json() {
        let tmp = TempDir::new().unwrap();
        let filename = tmp
            .path()
            .join("test.json")
            .into_os_string()
            .into_string()
            .unwrap();

        let json = read_json("tests/spanr/Atha.json");

        write_json(&filename, &json).expect("Write error");

        let lines = read_lines(&filename);
        assert!(lines.len() == 17 || lines.len() == 18);
    }
}

pub fn fields_to_idx(str: &str) -> Vec<usize> {
    let mut ints: Vec<i32> = vec![];
    let parts: Vec<&str> = str.split(',').collect();
    for p in parts {
        let intspan = IntSpan::from(p);
        intspan.elements().iter().for_each(|e| ints.push(*e));
    }

    ints.iter().map(|e| *e as usize).collect()
}

pub fn fields_to_ints(str: &str) -> IntSpan {
    let mut ints = IntSpan::new();
    let parts: Vec<&str> = str.split(',').collect();
    for p in parts {
        ints.add_runlist(p);
    }

    ints
}

// rewrite from https://metacpan.org/dist/Number-Format/source/Format.pm
pub fn format_number(number: f64, decimal_digits: usize) -> String {
    // Handle negative numbers
    let sign = if number < 0.0 { -1 } else { 1 };
    let mut number = number.abs();
    number = round(number, decimal_digits); // Round off number

    // Split integer and decimal parts of the number
    let integer_part = number.trunc() as i64;
    let decimal_part = number.fract();

    // Add the commas (fixed as `,`)
    let integer_str = integer_part.to_string();
    let formatted_integer = integer_str
        .chars()
        .rev()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(",")
        .chars()
        .rev()
        .collect::<String>();

    let decimal_str = format!("{:.1$}", decimal_part, decimal_digits)
        .trim_start_matches('0')
        .to_string();

    let result = if !decimal_str.is_empty() {
        format!("{}{}", formatted_integer, decimal_str)
    } else {
        formatted_integer
    };

    if sign < 0 {
        format!("-{}", result)
    } else {
        result
    }
}

fn round(number: f64, precision: usize) -> f64 {
    // Implement rounding logic
    (number * 10f64.powi(precision as i32)).round() / 10f64.powi(precision as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_number() {
        // Test positive numbers
        assert_eq!(format_number(1234567.89, 2), "1,234,567.89");
        assert_eq!(format_number(1000.0, 0), "1,000");
        assert_eq!(format_number(0.12345, 3), "0.123");

        // Test negative numbers
        assert_eq!(format_number(-9876543.21, 3), "-9,876,543.210");
        assert_eq!(format_number(-1000.0, 0), "-1,000");
        assert_eq!(format_number(-0.98765, 4), "-0.9877");

        // Test zero
        assert_eq!(format_number(0.0, 2), "0.00");
        assert_eq!(format_number(-0.0, 2), "0.00");

        // Test large numbers
        assert_eq!(format_number(1e10, 2), "10,000,000,000.00");
        assert_eq!(format_number(-1e10, 2), "-10,000,000,000.00");

        // Test decimal places
        assert_eq!(format_number(1234.56789, 3), "1,234.568");
        assert_eq!(format_number(1234.0, 5), "1,234.00000");
    }
}
