use std::fmt;
use std::vec::Vec;

pub struct IntSpan {
    edges: Vec<i32>,
    pos_inf: i32,
    neg_inf: i32,
    empty_string: String,
}

//----------------------------------------------------------
// Set contents
//----------------------------------------------------------

impl IntSpan {
    pub fn new() -> Self {
        IntSpan {
            edges: Vec::new(),
            pos_inf: 2147483647 - 1, // INT_MAX - 1, Real Largest int is POS_INF - 1
            neg_inf: -2147483648 + 1, // INT_MIN + 1
            empty_string: "-".to_string(),
        }
    }

    pub fn clear(&mut self) {
        self.edges.clear();
    }

    pub fn edge_size(&self) -> usize {
        self.edges.len()
    }

    pub fn span_size(&self) -> usize {
        self.edge_size() / 2
    }

    pub fn to_string(&self) -> String {
        if self.is_empty() {
            return self.empty_string.clone();
        }

        let mut runlist = "".to_string();

        for i in 0..self.span_size() {
            let lower = self.edges.get(i * 2).unwrap().clone();
            let upper = self.edges.get(i * 2 + 1).unwrap().clone() - 1;

            let mut buf = "".to_string();
            if i != 0 {
                buf.push_str(",");
            }

            if lower == upper {
                buf.push_str(lower.to_string().as_str());
            } else {
                buf.push_str( format!("{}-{}", lower, upper).as_str() );
            }

            runlist.push_str(buf.as_str());
        }

        runlist
    }

    pub fn ranges(&self) -> Vec<i32> {
        let mut ranges: Vec<i32> = Vec::new();

        for i in 0..self.edges.len() {
            // odd index means upper
            if (i & 1) == 1 {
                ranges.push(self.edges.get(i).unwrap().clone() - 1);
            } else {
                ranges.push(self.edges.get(i).unwrap().clone());
            }
        }

        ranges
    }

    pub fn contains(&self, n: i32) -> bool {
        let pos = self.find_pos(n + 1, 0);
        (pos & 1) == 1
    }

    pub fn min(&self) -> i32 {
        if self.is_empty() {
            panic!("Can't get extrema for empty IntSpan");
        }

        self.edges.first().unwrap().clone()
    }

    pub fn max(&self) -> i32 {
        if self.is_empty() {
            panic!("Can't get extrema for empty IntSpan");
        }

        self.edges.last().unwrap().clone() - 1
    }
}

//----------------------------------------------------------
// Set cardinality
//----------------------------------------------------------
impl IntSpan {
    pub fn cardinality(&self) -> i32 {
        let mut cardinality: i32 = 0;

        if self.is_empty() {
            return cardinality;
        }

        for i in 0..self.span_size() {
            let lower = self.edges.get(i * 2).unwrap().clone();
            let upper = self.edges.get(i * 2 + 1).unwrap().clone() - 1;

            cardinality += upper - lower + 1;
        }

        cardinality
    }

    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }

    pub fn is_neg_inf(&self) -> bool {
        self.edges.first().unwrap().clone() == self.neg_inf
    }

    pub fn is_pos_inf(&self) -> bool {
        self.edges.last().unwrap().clone() == self.pos_inf
    }

    pub fn is_infinite(&self) -> bool {
        self.is_neg_inf() || self.is_pos_inf()
    }

    pub fn is_finite(&self) -> bool {
        !self.is_infinite()
    }

    pub fn is_universal(&self) -> bool {
        self.edge_size() == 2 && self.is_pos_inf() && self.is_neg_inf()
    }
}

//----------------------------------------------------------
// Member operations (mutate original set)
//----------------------------------------------------------
impl IntSpan {
    pub fn add_pair(&mut self, mut lower: i32, mut upper: i32) {
        if lower > upper {
            panic!("Bad order: {},{}", lower, upper)
        }

        upper = upper + 1;

        let mut lower_pos = self.find_pos(lower, 0);
        let mut upper_pos = self.find_pos(upper + 1, lower_pos);

        if lower_pos & 1 == 1 {
            lower_pos = lower_pos - 1;
            lower = self.edges.get(lower_pos).unwrap().clone();
        }

        if upper_pos & 1 == 1 {
            upper = self.edges.get(upper_pos).unwrap().clone();
            upper_pos = upper_pos + 1;
        }

        for _i in lower_pos..upper_pos {
            self.edges.remove(lower_pos);
        }
        self.edges.insert(lower_pos, lower);
        self.edges.insert(lower_pos + 1, upper);
    }

    pub fn add_n(&mut self, n: i32) {
        self.add_pair(n, n);
    }

    pub fn add_range(&mut self, ranges: &Vec<i32>) {
        if ranges.len() % 2 != 0 {
            panic!("Number of ranges must be even")
        }

        // When this IntSpan is empty, just convert ranges to edges
        if self.is_empty() {
            for i in 0..ranges.len() {
                // odd index means upper
                if (i & 1) == 1 {
                    self.edges.push(ranges.get(i).unwrap().clone() + 1);
                } else {
                    self.edges.push(ranges.get(i).unwrap().clone());
                }
            }
        } else {
            for i in 0..(ranges.len() / 2) {
                let lower = ranges.get(i * 2).unwrap().clone();
                let upper = ranges.get(i * 2 + 1).unwrap().clone();

                self.add_pair(lower, upper);
            }
        }
    }

    pub fn merge(&mut self, other: &Self) {
        let ranges = other.ranges();

        self.add_range(&ranges);
    }

    pub fn add_vec(&mut self, ints: &Vec<i32>) {
        let ranges = self.list_to_ranges(ints);

        self.add_range(&ranges);
    }

    // https://hermanradtke.com/2015/05/06/creating-a-rust-function-that-accepts-string-or-str.html
    pub fn add_runlist<S>(&mut self, runlist: S)
    where
        S: Into<String>,
    {
        let s = runlist.into();
        // skip empty set
        if !s.is_empty() && !s.eq(&self.empty_string) {
            let ranges = self.runlist_to_ranges(&s);
            self.add_range(&ranges);
        }
    }

    pub fn invert(&mut self) {
        if self.is_empty() {
            // Universal set
            self.edges.push(self.neg_inf);
            self.edges.push(self.pos_inf);
        } else {
            // Either add or remove infinity from each end. The net effect is always an even number
            // of additions and deletions

            if self.is_neg_inf() {
                self.edges.remove(0); // shift
            } else {
                self.edges.insert(0, self.neg_inf); // unshift
            }

            if self.is_pos_inf() {
                self.edges.pop(); // pop
            } else {
                self.edges.push(self.pos_inf); // push
            }
        }
    }

    pub fn remove_pair(&mut self, lower: i32, upper: i32) {
        self.invert();
        self.add_pair(lower, upper);
        self.invert();
    }

    pub fn remove_n(&mut self, n: i32) {
        self.remove_pair(n, n);
    }

    pub fn remove_range(&mut self, ranges: &Vec<i32>) {
        if ranges.len() % 2 != 0 {
            panic!("Number of ranges must be even");
        }

        self.invert();
        self.add_range(ranges);
        self.invert();
    }

    pub fn subtract(&mut self, other: &Self) {
        let ranges = other.ranges();

        self.remove_range(&ranges);
    }

    pub fn remove_vec(&mut self, ints: &Vec<i32>) {
        let ranges = self.list_to_ranges(ints);

        self.remove_range(&ranges);
    }

    pub fn remove_runlist<S>(&mut self, runlist: S)
    where
        S: Into<String>,
    {
        let s = runlist.into();
        // skip empty set
        if !s.is_empty() && !s.eq(&self.empty_string) {
            let ranges = self.runlist_to_ranges(&s);
            self.remove_range(&ranges);
        }
    }
}

//----------------------------------------------------------
// Set binary operations (create new set)
//----------------------------------------------------------
impl IntSpan {
    pub fn copy(&self) -> Self {
        IntSpan {
            edges: self.edges.clone(),
            pos_inf: 2147483647 - 1, // INT_MAX - 1, Real Largest int is POS_INF - 1
            neg_inf: -2147483648 + 1, // INT_MIN + 1
            empty_string: "-".to_string(),
        }
    }

    pub fn union(&self, other: &Self) -> Self {
        let mut new = self.copy();
        new.merge(&other);
        new
    }

    pub fn complement(&self) -> Self {
        let mut new = self.copy();
        new.invert();
        new
    }

    pub fn diff(&self, other: &Self) -> Self {
        if self.is_empty() {
            Self::new()
        } else {
            let mut new = self.copy();
            new.subtract(&other);
            new
        }
    }

    pub fn intersect(&self, other: &Self) -> Self {
        if self.is_empty() || other.is_empty() {
            Self::new()
        } else {
            let mut new = self.complement();
            new.merge(&other.complement());
            new.invert();
            new
        }
    }

    pub fn xor(&self, other: &Self) -> Self {
        let mut new = self.union(&other);
        let intersect = self.intersect(&other);
        new.subtract(&intersect);
        new
    }
}

//----------------------------------------------------------
// Set relations
//----------------------------------------------------------
impl IntSpan {
    pub fn equals(&self, other: &Self) -> bool {
        let edges = &self.edges;
        let edges_other = &other.edges;

        if edges.len() != edges_other.len() {
            return false;
        }

        for i in 0..edges.len() {
            if edges.get(i) != edges_other.get(i) {
                return false;
            }
        }

        true
    }

    pub fn subset(&self, other: &Self) -> bool {
        self.diff(&other).is_empty()
    }

    pub fn superset(&self, other: &Self) -> bool {
        other.diff(&self).is_empty()
    }
}

//----------------------------------------------------------
// Private methods
//----------------------------------------------------------

impl IntSpan {
    fn find_pos(&self, val: i32, mut low: usize) -> usize {
        let mut high = self.edge_size();

        while low < high {
            let mid = (low + high) / 2;
            if val < self.edges.get(mid).unwrap().clone() {
                high = mid;
            } else if val > self.edges.get(mid).unwrap().clone() {
                low = mid + 1;
            } else {
                return mid;
            }
        }

        low
    }

    fn list_to_ranges(&self, ints: &Vec<i32>) -> Vec<i32> {
        let mut ranges: Vec<i32> = Vec::new();

        let mut ints = ints.clone();
        ints.sort_unstable();
        ints.dedup();

        let len = ints.len();
        let mut pos: usize = 0;

        while pos < len {
            let mut end = pos + 1;
            while (end < len) && (ints[end] <= ints[end - 1] + 1) {
                end = end + 1;
            }
            ranges.push(ints[pos]);
            ranges.push(ints[end - 1]);
            pos = end;
        }

        ranges
    }

    fn runlist_to_ranges(&self, runlist: &String) -> Vec<i32> {
        let mut ranges: Vec<i32> = Vec::new();

        let bytes = runlist.as_bytes();

        let radix = 10;
        let mut idx = 0; // index in runlist
        let len = bytes.len();

        let mut lower_is_neg = false;
        let mut upper_is_neg = false;
        let mut in_upper = false;

        while idx < len {
            let mut i = 0; // index in one run
            if bytes.get(idx).unwrap().clone() == '-' as u8 {
                lower_is_neg = true;
                i += 1;
            }

            // ported from Java Integer.parseInt()
            let mut lower: i32 = 0;
            let mut upper: i32 = 0;

            while idx + i < len {
                let ch = bytes[idx + i];
                if ch >= '0' as u8 && ch <= '9' as u8 {
                    if !in_upper {
                        lower *= radix;
                        lower -= (ch as char).to_digit(10).unwrap() as i32;
                    } else {
                        upper *= radix;
                        upper -= (ch as char).to_digit(10).unwrap() as i32;
                    }
                } else if ch == '-' as u8 && !in_upper {
                    in_upper = true;
                    if bytes.get(idx + i + 1).unwrap().clone() == '-' as u8 {
                        upper_is_neg = true;
                    }
                } else if ch == ',' as u8 {
                    i = i + 1;
                    break; // end of run
                }

                i = i + 1;
            }

            if !in_upper {
                ranges.push(if lower_is_neg { lower } else { -lower }); // add lower
                ranges.push(if lower_is_neg { lower } else { -lower }); // add lower again
            } else {
                ranges.push(if lower_is_neg { lower } else { -lower }); // add lower
                ranges.push(if upper_is_neg { upper } else { -upper }); // add lower
            }

            // reset boolean flags
            lower_is_neg = false;
            upper_is_neg = false;
            in_upper = false;

            // start next run
            idx += i;
        }

        ranges
    }
}

impl fmt::Display for IntSpan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())?;
        Ok(())
    }
}
