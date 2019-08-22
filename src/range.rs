use regex::Regex;
use std::collections::BTreeMap;

#[derive(Default)]
pub struct Range {
    name: String,
    chr: String,
    strand: String,
    start: i32,
    end: i32,
    others: BTreeMap<String, String>,
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
    pub fn chr_mut(&mut self) -> &mut String {
        &mut self.chr
    }
    pub fn strand_mut(&mut self) -> &mut String {
        &mut self.strand
    }
    pub fn start_mut(&mut self) -> &mut i32 {
        &mut self.start
    }
    pub fn end_mut(&mut self) -> &mut i32 {
        &mut self.end
    }

    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            chr: "".to_string(),
            strand: "".to_string(),
            start: 0,
            end: 0,
            others: BTreeMap::new(),
        }
    }

    /// From chr, start and end
    ///
    /// ```
    /// # use intspan::Range;
    /// let mut range = Range::from("I", 1, 100);
    /// # assert_eq!(*range.chr(), "I");
    /// # assert_eq!(*range.start(), 1);
    /// # assert_eq!(*range.end(), 100);
    /// ```
    pub fn from<S>(chr: S, start: i32, end: i32) -> Self
    where
        S: Into<String>,
    {
        let s = chr.into();

        Self {
            name: "".to_string(),
            chr: s,
            strand: "".to_string(),
            start,
            end,
            others: BTreeMap::new(),
        }
    }

    pub fn from_str<S>(range: S) -> Self
    where
        S: Into<String>,
    {
        let s = range.into();

        let mut new = Self::new();

        new
    }
}
