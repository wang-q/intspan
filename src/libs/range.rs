use crate::IntSpan;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;

#[derive(Default, Clone)]
pub struct Range {
    pub name: String,
    pub chr: String,
    pub strand: String,
    pub start: i32,
    pub end: i32,
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
    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
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
        Self {
            name: "".to_string(),
            chr: chr.to_string(),
            strand: "".to_string(),
            start,
            end,
        }
    }

    /// Constructed from chr, start and end
    ///
    /// ```
    /// # use intspan::Range;
    /// let range = Range::from_full("S288c", "I", "-", 1, 100);
    /// # assert_eq!(*range.name(), "S288c");
    /// # assert_eq!(*range.chr(), "I");
    /// # assert_eq!(*range.strand(), "-");
    /// # assert_eq!(*range.start(), 1);
    /// # assert_eq!(*range.end(), 100);
    /// ```
    pub fn from_full(name: &str, chr: &str, strand: &str, start: i32, end: i32) -> Self {
        Self {
            name: name.to_string(),
            chr: chr.to_string(),
            strand: strand.to_string(),
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
        let mut new = Self::new();
        new.decode(range);

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

    /// Trim both ends
    ///
    /// ```
    /// # use intspan::Range;
    /// let range = Range::from_str("I:100-200");
    /// assert_eq!(range.trim(30).to_string(), "I:130-170");
    /// assert_eq!(range.trim(70).is_valid(), false);
    /// assert_eq!(range.trim(-30).to_string(), "I:70-230");
    /// ```
    pub fn trim(&self, n: i32) -> Self {
        let mut start = self.start + n;
        let mut end = self.end - n;
        Self::check(&mut start, &mut end);

        Self {
            name: self.name.to_string(),
            chr: self.chr.to_string(),
            strand: self.strand.to_string(),
            start,
            end,
        }
    }

    /// Trim 5p end
    ///
    /// ```
    /// # use intspan::Range;
    /// let range = Range::from_str("I(+):100-200");
    /// assert_eq!(range.trim_5p(30).to_string(), "I(+):130-200");
    /// let range = Range::from_str("I(-):100-200");
    /// assert_eq!(range.trim_5p(30).to_string(), "I(-):100-170");
    /// assert_eq!(range.trim_5p(-30).to_string(), "I(-):100-230");
    /// assert_eq!(range.trim_5p(120).is_valid(), false);
    /// ```
    pub fn trim_5p(&self, n: i32) -> Self {
        let mut start = if self.strand == "-" {
            self.start
        } else {
            self.start + n
        };
        let mut end = if self.strand == "-" {
            self.end - n
        } else {
            self.end
        };
        Self::check(&mut start, &mut end);

        Self {
            name: self.name.to_string(),
            chr: self.chr.to_string(),
            strand: self.strand.to_string(),
            start,
            end,
        }
    }

    /// Trim 3p end
    ///
    /// ```
    /// # use intspan::Range;
    /// let range = Range::from_str("I(+):100-200");
    /// assert_eq!(range.trim_3p(30).to_string(), "I(+):100-170");
    /// let range = Range::from_str("I(-):100-200");
    /// assert_eq!(range.trim_3p(30).to_string(), "I(-):130-200");
    /// assert_eq!(range.trim_3p(120).is_valid(), false);
    /// ```
    pub fn trim_3p(&self, n: i32) -> Self {
        let mut start = if self.strand == "-" {
            self.start + n
        } else {
            self.start
        };
        let mut end = if self.strand == "-" {
            self.end
        } else {
            self.end - n
        };
        Self::check(&mut start, &mut end);

        Self {
            name: self.name.to_string(),
            chr: self.chr.to_string(),
            strand: self.strand.to_string(),
            start,
            end,
        }
    }

    /// Shift to 5p end
    ///
    /// ```
    /// # use intspan::Range;
    /// let range = Range::from_str("I(+):100-200");
    /// assert_eq!(range.shift_5p(30).to_string(), "I(+):70-170");
    /// assert_eq!(range.shift_5p(-30).to_string(), "I(+):130-230");
    /// let range = Range::from_str("I(-):100-200");
    /// assert_eq!(range.shift_5p(30).to_string(), "I(-):130-230");
    /// ```
    pub fn shift_5p(&self, n: i32) -> Self {
        let mut start = if self.strand == "-" {
            self.start + n
        } else {
            self.start - n
        };
        let mut end = if self.strand == "-" {
            self.end + n
        } else {
            self.end - n
        };
        Self::check(&mut start, &mut end);

        Self {
            name: self.name.to_string(),
            chr: self.chr.to_string(),
            strand: self.strand.to_string(),
            start,
            end,
        }
    }

    /// Shift to 3p end
    ///
    /// ```
    /// # use intspan::Range;
    /// let range = Range::from_str("I(+):100-200");
    /// assert_eq!(range.shift_3p(30).to_string(), "I(+):130-230");
    /// assert_eq!(range.shift_3p(-30).to_string(), "I(+):70-170");
    /// let range = Range::from_str("I(-):100-200");
    /// assert_eq!(range.shift_3p(30).to_string(), "I(-):70-170");
    /// ```
    pub fn shift_3p(&self, n: i32) -> Self {
        self.shift_5p(-n)
    }

    /// Flanking region of the 5p end.
    /// A negative value for 'n' indicates positions within the range.
    ///
    /// ```
    /// # use intspan::Range;
    /// let range = Range::from_str("I(+):100-200");
    /// assert_eq!(range.flank_5p(30).to_string(), "I(+):70-99");
    /// assert_eq!(range.flank_5p(-30).to_string(), "I(+):100-129");
    /// assert_eq!(range.flank_5p(0).is_valid(), false);
    /// let range = Range::from_str("I(-):100-200");
    /// assert_eq!(range.flank_5p(30).to_string(), "I(-):201-230");
    /// assert_eq!(range.flank_5p(-30).to_string(), "I(-):171-200");
    /// assert_eq!(range.flank_5p(0).is_valid(), false);
    /// ```
    pub fn flank_5p(&self, n: i32) -> Self {
        let mut start = if n > 0 {
            if self.strand == "-" {
                self.end + 1
            } else {
                self.start - n
            }
        } else if self.strand == "-" {
            self.end + n + 1
        } else {
            self.start
        };
        let mut end = if n > 0 {
            if self.strand == "-" {
                self.end + n
            } else {
                self.start - 1
            }
        } else if self.strand == "-" {
            self.end
        } else {
            self.start - n - 1
        };
        Self::check(&mut start, &mut end);

        Self {
            name: self.name.to_string(),
            chr: self.chr.to_string(),
            strand: self.strand.to_string(),
            start,
            end,
        }
    }

    /// Flanking region of the 3p end
    ///
    /// ```
    /// # use intspan::Range;
    /// let range = Range::from_str("I(+):100-200");
    /// assert_eq!(range.flank_3p(30).to_string(), "I(+):201-230");
    /// assert_eq!(range.flank_3p(-30).to_string(), "I(+):171-200");
    /// assert_eq!(range.flank_3p(0).is_valid(), false);
    /// let range = Range::from_str("I(-):100-200");
    /// assert_eq!(range.flank_3p(30).to_string(), "I(-):70-99");
    /// assert_eq!(range.flank_3p(-30).to_string(), "I(-):100-129");
    /// assert_eq!(range.flank_3p(0).is_valid(), false);
    /// ```
    pub fn flank_3p(&self, n: i32) -> Self {
        let mut start = if n > 0 {
            if self.strand == "-" {
                self.start - n
            } else {
                self.end + 1
            }
        } else if self.strand == "-" {
            self.start
        } else {
            self.end + n + 1
        };
        let mut end = if n > 0 {
            if self.strand == "-" {
                self.start - 1
            } else {
                self.end + n
            }
        } else if self.strand == "-" {
            self.start - n - 1
        } else {
            self.end
        };
        Self::check(&mut start, &mut end);

        Self {
            name: self.name.to_string(),
            chr: self.chr.to_string(),
            strand: self.strand.to_string(),
            start,
            end,
        }
    }

    fn decode(&mut self, header: &str) {
        let caps = match RE.captures(header) {
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

    fn check(start: &mut i32, end: &mut i32) {
        if *start < 0 {
            *start = 0;
        }
        if *end < 0 {
            *end = 0;
        }
        if *start > *end {
            *start = 0;
            *end = 0;
        }
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
