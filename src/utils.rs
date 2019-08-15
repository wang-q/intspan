use std::fs;
use std::io::{self, BufRead, BufReader, Lines};

pub fn reader(input: &String) -> Box<BufRead> {
    let reader: Box<dyn BufRead> = if input == "stdin" {
        Box::new(BufReader::new(io::stdin()))
    } else {
        Box::new(BufReader::new(fs::File::open(input).unwrap()))
    };

    reader
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn test_stdin() {
//        let cursor = io::Cursor::new(b"lorem\nipsum\ndolor\n");
//        let mut lines_iter =
//            reader(&"stdin".to_string()).lines().filter_map(io::Result::ok).collect();
////        assert_eq!(lines_iter.next(), Some(String::from("lorem")));
////        assert_eq!(lines_iter.next(), Some(String::from("ipsum")));
////        assert_eq!(lines_iter.next(), Some(String::from("dolor")));
////        assert_eq!(lines_iter.next(), None);
//        println!("{:?}", lines_iter);
//    }
//}
