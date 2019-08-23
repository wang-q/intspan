use intspan::IntSpan;
use serde_yaml::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};

pub fn reader(input: &str) -> Box<dyn BufRead> {
    let reader: Box<dyn BufRead> = if input == "stdin" {
        Box::new(BufReader::new(io::stdin()))
    } else {
        Box::new(BufReader::new(fs::File::open(input).unwrap()))
    };

    reader
}

pub fn read_lines(input: &str) -> Vec<String> {
    let mut reader = reader(input);
    let mut s = String::new();
    reader.read_to_string(&mut s);
    s.lines().map(|s| s.to_string()).collect::<Vec<String>>()
}

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

pub fn read_yaml(input: &str) -> BTreeMap<String, Value> {
    let mut reader = reader(input);
    let mut s = String::new();
    reader.read_to_string(&mut s);

    serde_yaml::from_str(&s).unwrap()
}

pub fn writer(output: &str) -> Box<dyn Write> {
    let writer: Box<dyn Write> = if output == "stdout" {
        Box::new(BufWriter::new(io::stdout()))
    } else {
        Box::new(BufWriter::new(fs::File::create(output).unwrap()))
    };

    writer
}

pub fn write_lines(output: &str, lines: &Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = writer(output);

    for line in lines {
        writer.write_all(format!("{}\n", line).as_ref())?;
    }

    Ok(())
}

pub fn write_yaml(
    output: &str,
    yaml: &BTreeMap<String, Value>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = writer(output);
    let mut s = serde_yaml::to_string(yaml).unwrap();
    s.push_str("\n");
    writer.write_all(s.as_bytes())?;

    Ok(())
}

pub fn yaml2set(yaml: &BTreeMap<String, Value>) -> BTreeMap<String, IntSpan> {
    let mut set: BTreeMap<String, IntSpan> = BTreeMap::new();

    for (chr, value) in yaml {
        let intspan = IntSpan::from(value.as_str().unwrap());
        set.insert(chr.into(), intspan);
    }

    set
}

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
    for (_name, set) in set_of {
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

#[cfg(test)]
mod read_write {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_reader() {
        let reader = reader("tests/resources/S288c.chr.sizes");
        let mut lines = vec![];
        for line in reader.lines() {
            lines.push(line);
        }
        assert_eq!(lines.len(), 16);
    }

    #[test]
    fn test_reader_2() {
        let reader = reader("tests/resources/S288c.chr.sizes");
        assert_eq!(reader.lines().collect::<Vec<_>>().len(), 16);
    }

    #[test]
    fn test_read_lines() {
        let lines = read_lines("tests/resources/S288c.chr.sizes");
        assert_eq!(lines.len(), 16);
    }

    #[test]
    fn test_read_sizes() {
        let sizes = read_sizes("tests/resources/S288c.chr.sizes");
        assert_eq!(sizes.len(), 16);
        assert_eq!(*sizes.get("II").unwrap(), 813184);
    }

    #[test]
    fn test_write_lines() {
        let tempdir = TempDir::new().unwrap();
        let filename = tempdir
            .path()
            .join("test.txt")
            .into_os_string()
            .into_string()
            .unwrap();
        write_lines(&filename, &vec!["This", "is", "a\ntest"]);

        let lines = read_lines(&filename);
        assert_eq!(lines.len(), 4);
    }

    #[test]
    fn test_read_write_runlist() {
        let tempdir = TempDir::new().unwrap();
        let filename = tempdir
            .path()
            .join("test.yml")
            .into_os_string()
            .into_string()
            .unwrap();

        let yaml = read_yaml("tests/resources/Atha.yml");

        write_yaml(&filename, &yaml);

        let lines = read_lines(&filename);
        assert_eq!(lines.len(), 11);
    }

    #[test]
    fn test_yaml2set() {
        let value: Value = serde_yaml::to_value("28547-29194").unwrap();
        let mut runlist_of: BTreeMap<String, Value> = BTreeMap::new();
        runlist_of.insert("I".to_string(), value);

        let set_of = yaml2set(&runlist_of);
        assert!(set_of.values().next().unwrap().contains(28550));
    }

    #[test]
    fn test_set2yaml() {
        let mut intspan = IntSpan::new();
        intspan.add_pair(28547, 29194);
        let mut set_of: BTreeMap<String, IntSpan> = BTreeMap::new();
        set_of.insert("I".to_string(), intspan);

        let runlist_of = set2yaml(&set_of);
        assert_eq!(
            runlist_of.values().next().unwrap(),
            &Value::String("28547-29194".into())
        );
    }
}
