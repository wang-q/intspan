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

    pub fn clear(&mut self) -> &Self {
        self.edges.clear();

        self
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
                buf.push_str(lower.to_string().as_str());
                buf.push_str("-");
                buf.push_str(upper.to_string().as_str());
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
    pub fn add_pair(&mut self, mut lower: i32, mut upper: i32) -> &Self {
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

        self
    }

    pub fn add_n(&mut self, n: i32) -> &Self {
        self.add_pair(n, n)
    }

    pub fn add_range(&mut self, ranges: Vec<i32>) -> &Self {
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

        self
    }

    pub fn merge(&mut self, supplied: IntSpan) -> &Self {
        let ranges = supplied.ranges();

        self.add_range(ranges)
    }

    pub fn add_vec(&mut self, ints: Vec<i32>) -> &Self {
        let ranges = self.list_to_ranges(ints);

        self.add_range(ranges)
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

    fn list_to_ranges(&self, mut ints: Vec<i32>) -> Vec<i32> {
        ints.sort_unstable();
        ints.dedup();

        let mut ranges: Vec<i32> = Vec::new();

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
}

impl fmt::Display for IntSpan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())?;
        Ok(())
    }
}
