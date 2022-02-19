use crate::{IntSpan, Range};
use flate2::read::GzDecoder;
use serde_yaml::Value;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::path::Path;

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
///
/// let reader = intspan::reader("tests/far/ufasta.fa.gz");
/// assert_eq!(reader.lines().collect::<Vec<_>>().len(), 256);
/// ```
pub fn reader(input: &str) -> Box<dyn BufRead> {
    let reader: Box<dyn BufRead> = if input == "stdin" {
        Box::new(BufReader::new(io::stdin()))
    } else {
        let path = Path::new(input);
        let file = match File::open(&path) {
            Err(why) => panic!("could not open {}: {}", path.display(), why.to_string()),
            Ok(file) => file,
        };

        if path.extension() == Some(OsStr::new("gz")) {
            Box::new(BufReader::new(GzDecoder::new(file)))
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
/// let sizes = intspan::read_first_column("tests/spanr/S288c.chr.sizes");
/// assert_eq!(sizes.len(), 16);
/// assert_eq!(*sizes.get(1).unwrap(), "II");
/// assert_eq!(*sizes.get(15).unwrap(), "XVI");
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

pub fn read_yaml(input: &str) -> BTreeMap<String, Value> {
    let mut reader = reader(input);
    let mut s = String::new();
    reader.read_to_string(&mut s).expect("Read error");

    serde_yaml::from_str(&s).unwrap()
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

pub fn write_yaml(output: &str, yaml: &BTreeMap<String, Value>) -> Result<(), std::io::Error> {
    let mut writer = writer(output);
    let mut s = serde_yaml::to_string(yaml).unwrap();
    s.push_str("\n");
    writer.write_all(s.as_bytes())?;

    Ok(())
}

/// ```
/// use serde_yaml::Value;
/// use std::collections::BTreeMap;
/// let value: Value = serde_yaml::to_value("28547-29194").unwrap();
/// let mut runlists: BTreeMap<String, Value> = BTreeMap::new();
/// runlists.insert("I".to_string(), value);
///
/// let sets = intspan::yaml2set(&runlists);
/// assert!(sets.values().next().unwrap().contains(28550));
/// ```
pub fn yaml2set(yaml: &BTreeMap<String, Value>) -> BTreeMap<String, IntSpan> {
    let mut set: BTreeMap<String, IntSpan> = BTreeMap::new();

    for (chr, value) in yaml {
        let intspan = IntSpan::from(value.as_str().unwrap());
        set.insert(chr.into(), intspan);
    }

    set
}

/// ```
/// use serde_yaml::Value;
/// use std::collections::BTreeMap;
/// use intspan::IntSpan;
/// let mut intspan = IntSpan::new();
/// intspan.add_pair(28547, 29194);
/// let mut set_of: BTreeMap<String, IntSpan> = BTreeMap::new();
/// set_of.insert("I".to_string(), intspan);
///
/// let runlist_of = intspan::set2yaml(&set_of);
/// assert_eq!(
///     runlist_of.values().next().unwrap(),
///     &Value::String("28547-29194".into())
/// );
/// ```
pub fn set2yaml(set: &BTreeMap<String, IntSpan>) -> BTreeMap<String, Value> {
    let mut yaml: BTreeMap<String, Value> = BTreeMap::new();

    for (chr, value) in set {
        let runlist = value.to_string();
        yaml.insert(chr.into(), serde_yaml::to_value(runlist).unwrap());
    }

    yaml
}

pub fn set2yaml_m(set_of: &BTreeMap<String, BTreeMap<String, IntSpan>>) -> BTreeMap<String, Value> {
    let mut out_yaml: BTreeMap<String, Value> = BTreeMap::new();

    for (name, set) in set_of {
        let yaml = set2yaml(set);
        out_yaml.insert(name.to_string(), serde_yaml::to_value(yaml).unwrap());
    }

    out_yaml
}

pub fn yaml2set_m(yaml: &BTreeMap<String, Value>) -> BTreeMap<String, BTreeMap<String, IntSpan>> {
    let is_multi: bool = yaml.values().next().unwrap().is_mapping();

    let mut s_of: BTreeMap<String, BTreeMap<String, IntSpan>> = BTreeMap::new();
    if is_multi {
        for (key, value) in yaml {
            let string = serde_yaml::to_string(value).unwrap();
            let runlist_one: BTreeMap<String, Value> =
                serde_yaml::from_str(string.as_str()).unwrap();
            let set_one = yaml2set(&runlist_one);
            s_of.insert(key.to_string(), set_one);
        }
    } else {
        let set_one = yaml2set(&yaml);
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
        among_links.sort_by_cached_key(|k| {
            let parts: Vec<&str> = k.split('\t').collect();
            Reverse(parts.len())
        });
    }

    among_links
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
    fn test_read_write_runlist() {
        let tmp = TempDir::new().unwrap();
        let filename = tmp
            .path()
            .join("test.yml")
            .into_os_string()
            .into_string()
            .unwrap();

        let yaml = read_yaml("tests/spanr/Atha.yml");

        write_yaml(&filename, &yaml).expect("Write error");

        let lines = read_lines(&filename);
        assert!(lines.len() == 11 || lines.len() == 12);
    }
}
