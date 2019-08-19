use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead, BufReader, Lines, Read};

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

//pub fn read_sizes(input: &String) -> HashMap<String, i32> {
//
//}

#[cfg(test)]
mod tests {
    use super::*;

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
}
