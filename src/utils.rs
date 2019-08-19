use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead, BufReader, BufWriter, Lines, Read, Write};

pub fn reader(input: &String) -> Box<dyn BufRead> {
    let reader: Box<dyn BufRead> = if input == "stdin" {
        Box::new(BufReader::new(io::stdin()))
    } else {
        Box::new(BufReader::new(fs::File::open(input).unwrap()))
    };

    reader
}

pub fn read_lines(input: &String) -> Vec<String> {
    let mut reader = reader(input);
    let mut s = String::new();
    reader.read_to_string(&mut s);
    s.lines().map(|s| s.to_string()).collect::<Vec<String>>()
}

pub fn read_sizes(input: &String) -> HashMap<String, i32> {
    let mut length_of: HashMap<String, i32> = HashMap::new();

    for line in read_lines(input) {
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() == 2 {
            length_of.insert(fields[0].to_string(), fields[1].parse::<i32>().unwrap());
        }
    }

    length_of
}

pub fn writer(output: &String) -> Box<dyn Write> {
    let writer: Box<dyn Write> = if output == "stdout" {
        Box::new(BufWriter::new(io::stdout()))
    } else {
        Box::new(BufWriter::new(fs::File::create(output).unwrap()))
    };

    writer
}

pub fn write_lines(output: &String, lines: Vec<&str>) {
    let mut writer = writer(output);

    for line in lines {
        writer.write(format!("{}\n", line).as_ref());
    }
}

#[cfg(test)]
mod read_write {
    use super::*;
    use tempfile::{tempdir, Builder, TempDir};

    #[test]
    fn test_reader() {
        let reader = reader(&"tests/resources/S288c.chr.sizes".to_string());
        let mut lines = vec![];
        for line in reader.lines() {
            lines.push(line);
        }
        assert_eq!(lines.len(), 16);
    }

    #[test]
    fn test_reader_2() {
        let reader = reader(&"tests/resources/S288c.chr.sizes".to_string());
        let lines: Vec<_> = reader.lines().collect();
        assert_eq!(lines.len(), 16);
    }

    #[test]
    fn test_read_lines() {
        let lines = read_lines(&"tests/resources/S288c.chr.sizes".to_string());
        assert_eq!(lines.len(), 16);
    }

    #[test]
    fn test_read_sizes() {
        let length_of = read_sizes(&"tests/resources/S288c.chr.sizes".to_string());
        assert_eq!(length_of.len(), 16);
        assert_eq!(length_of.get("II").unwrap(), &813184);
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
        write_lines(&filename, vec!["This", "is", "a\ntest"]);

        println!("1");
        let lines = read_lines(&filename);
        assert_eq!(lines.len(), 4);
    }

}
