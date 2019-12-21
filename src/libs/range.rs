use crate::IntSpan;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;

#[derive(Default, Clone)]
pub struct Range {
    name: String,
    chr: String,
    strand: String,
    start: i32,
    end: i32,
}

lazy_static! {
    static ref RE: Regex = Regex::new(
        r"(?xi)
        (?:(?P<name>[\w_]+)\.)?
        (?P<chr>[\w/-]+)
        (?:\((?P<strand>.+)\))?
        [:]                    # spacer
        (?P<start>\d+)
        [_\-]?                 # spacer
        (?P<end>\d+)?
        ",
    )
    .unwrap();
}

impl Range {
    // Immutable accessors
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn chr(&self) -> &String {
        &self.chr
    }
    pub fn strand(&self) -> &String {
        &self.strand
    }
    pub fn start(&self) -> &i32 {
        &self.start
    }
    pub fn end(&self) -> &i32 {
        &self.end
    }

    // Mutable accessors
    pub fn strand_mut(&mut self) -> &mut String {
        &mut self.strand
    }

    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            chr: "".to_string(),
            strand: "".to_string(),
            start: 0,
            end: 0,
        }
    }

    /// Constructed from chr, start and end
    ///
    /// ```
    /// # use intspan::Range;
    /// let range = Range::from("I", 1, 100);
    /// # assert_eq!(*range.chr(), "I");
    /// # assert_eq!(*range.start(), 1);
    /// # assert_eq!(*range.end(), 100);
    /// ```
    pub fn from(chr: &str, start: i32, end: i32) -> Self {
        let s = chr.into();

        Self {
            name: "".to_string(),
            chr: s,
            strand: "".to_string(),
            start,
            end,
        }
    }

    /// Constructed from string
    ///
    /// ```
    /// # use intspan::Range;
    /// let range = Range::from_str("I:1-100");
    /// # assert_eq!(*range.chr(), "I");
    /// # assert_eq!(*range.start(), 1);
    /// # assert_eq!(*range.end(), 100);
    /// # assert_eq!(range.to_string(), "I:1-100");
    /// let range = Range::from_str("I:100");
    /// # assert_eq!(*range.chr(), "I");
    /// # assert_eq!(*range.start(), 100);
    /// # assert_eq!(*range.end(), 100);
    /// # assert_eq!(range.to_string(), "I:100");
    /// let range = Range::from_str("S288c.I(-):27070-29557");
    /// # assert_eq!(*range.name(), "S288c");
    /// # assert_eq!(*range.strand(), "-");
    /// # assert_eq!(range.to_string(), "S288c.I(-):27070-29557");
    /// ```
    pub fn from_str(range: &str) -> Self {
        let s = range.into();

        let mut new = Self::new();
        new.decode(&s);

        new
    }

    /// Valid or not
    ///
    /// ```
    /// # use intspan::Range;
    /// let range = Range::from("I", 1, 100);
    /// assert!(range.is_valid());
    /// let range = Range::from_str("I:100");
    /// assert!(range.is_valid());
    /// let range = Range::from_str("invalid");
    /// assert!(!range.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        self.start != 0
    }

    /// IntSpan
    ///
    /// ```
    /// # use intspan::Range;
    /// let range = Range::from("I", 1, 100);
    /// assert_eq!(range.intspan().to_string(), "1-100");
    /// let range = Range::from_str("I:100");
    /// assert_eq!(range.intspan().to_string(), "100");
    /// ```
    pub fn intspan(&self) -> IntSpan {
        IntSpan::from_pair(self.start, self.end)
    }

    fn decode(&mut self, header: &String) {
        let caps = match RE.captures(header.as_str()) {
            Some(x) => x,
            None => {
                self.chr = header.split(' ').next().unwrap().to_string();
                return;
            }
        };
        let dict: HashMap<String, String> = RE
            .capture_names()
            .flatten()
            .filter_map(|n| Some((n.to_string(), caps.name(n)?.as_str().to_string())))
            .collect();
        for key in dict.keys() {
            match key.as_str() {
                "name" => self.name = dict.get(key).unwrap().to_owned(),
                "chr" => self.chr = dict.get(key).unwrap().to_owned(),
                "strand" => self.strand = dict.get(key).unwrap().to_owned(),
                "start" => self.start = dict.get(key).unwrap().parse::<i32>().unwrap(),
                "end" => self.end = dict.get(key).unwrap().parse::<i32>().unwrap(),
                _ => {}
            }
        }

        if self.start != 0 && self.end == 0 {
            self.end = self.start;
        }

        //        eprintln!("{:#?}", &dict);
    }

    fn encode(&self) -> String {
        let mut header = String::new();

        if !self.name.is_empty() {
            header += self.name.as_str();
            if !self.chr.is_empty() {
                header += ".";
                header += self.chr.as_str();
            }
        } else if !self.chr.is_empty() {
            header += self.chr.as_str();
        }

        if !self.strand.is_empty() {
            header += "(";
            header += self.strand.as_str();
            header += ")";
        }

        if self.start != 0 {
            header += ":";
            header += self.start.to_string().as_str();
            if self.end != self.start {
                header += "-";
                header += self.end.to_string().as_str();
            }
        }

        header
    }
}

/// To string
///
/// ```
/// # use intspan::Range;
/// let range = Range::from("I", 1, 100);
/// assert_eq!(range.to_string(), "I:1-100");
/// let range = Range::from("I", 100, 100);
/// assert_eq!(range.to_string(), "I:100");
/// ```
impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.encode())?;
        Ok(())
    }
}

#[test]
fn fa_headers() {
    let tests = vec![
        ("S288c", "S288c"),
        ("S288c The baker's yeast", "S288c"),
        ("1:-100", "1:-100"),
        ("infile_0/1/0_514:19-25", "infile_0/1/0_514:19-25"),
    ];
    for (header, expected) in tests {
        let range = Range::from_str(header);
        assert_eq!(range.to_string(), expected);
    }
}
