//! `IntSpan` handles of sets containing integer spans.
//!
//! # SYNOPSIS
//!
//! ```
//! use intspan::IntSpan;
//!
//! let mut set = IntSpan::new();
//! for i in vec![1, 2, 3, 5, 7, 9] {
//!     set.add_n(i);
//! }
//! set.add_pair(100, 10000);
//! set.remove_n(1000);
//!
//! let expected = "1-3,5,7,9,100-999,1001-10000";
//! # assert_eq!(set.to_string(), expected);
//! # assert_eq!(set.cardinality(), 9906);
//! # assert_eq!(set.is_empty(), false);
//! # assert_eq!(set.is_universal(), false);
//! # assert_eq!(set.is_infinite(), false);
//! # assert_eq!(set.is_finite(), true);
//! # assert_eq!(set.is_pos_inf(), false);
//! # assert_eq!(set.is_neg_inf(), false);
//! ```
//!
//! ```
//! # use intspan::IntSpan;
//! let set = IntSpan::from("1-3,5,7,9,100-999,1001-10000");
//! # assert_eq!(set.to_string(), "1-3,5,7,9,100-999,1001-10000");
//! # assert_eq!(set.cardinality(), 9906);
//! ```
//!
//! # DESCRIPTION
//!
//! `IntSpan` represents sets of integers as a number of inclusive ranges, for example
//! `1-10,19-23,45-48`. Because many of its operations involve linear searches of the list of ranges its
//! overall performance tends to be proportional to the number of distinct ranges. This is fine for
//! small sets but suffers compared to other possible set representations (bit vectors, hash keys) when
//! the number of ranges grows large.
//!
//! This module also represents sets as ranges of values but stores those ranges in order and uses a
//! binary search for many internal operations so that overall performance tends towards O log N where N
//! is the number of ranges.
//!
//! The internal representation used by this module is extremely simple: a set is represented as a list
//! of integers. Integers in even numbered positions (0, 2, 4 etc) represent the start of a run of
//! numbers while those in odd numbered positions represent the ends of runs. As an example the set (1,
//! 3-7, 9, 11, 12) would be represented internally as (1, 2, 3, 8, 11, 13).
//!
//! Sets may be infinite - assuming you're prepared to accept that infinity is actually no more than a
//! fairly large integer. Specifically the constants `neg_inf` and `pos_inf` are defined to be (-2^31+1)
//! and (2^31-2) respectively. To create an infinite set invert an empty one:
//!
//! ```
//! # use intspan::IntSpan;
//! let mut set = IntSpan::new();
//! set.invert();
//! # let expected = format!("{}-{}", set.get_neg_inf(), set.get_pos_inf());
//! # assert_eq!(set.to_string(), expected);
//! # assert_eq!(set.is_empty(), false);
//! # assert_eq!(set.is_universal(), true);
//! # assert_eq!(set.is_infinite(), true);
//! # assert_eq!(set.is_finite(), false);
//! # assert_eq!(set.is_pos_inf(), true);
//! # assert_eq!(set.is_neg_inf(), true);
//! ```
//!
//! Sets need only be bounded in one direction - for example this is the set of all positive integers
//! (assuming you accept the slightly feeble definition of infinity we're using):
//!
//! ```
//! # use intspan::IntSpan;
//! let mut set = IntSpan::new();
//! set.add_pair(1, set.get_pos_inf());
//! # let expected = format!("{}-{}", 1, set.get_pos_inf());
//! # assert_eq!(set.to_string(), expected);
//! # assert_eq!(set.is_empty(), false);
//! # assert_eq!(set.is_universal(), false);
//! # assert_eq!(set.is_infinite(), true);
//! # assert_eq!(set.is_finite(), false);
//! # assert_eq!(set.is_pos_inf(), true);
//! # assert_eq!(set.is_neg_inf(), false);
//! ```
//!
//! This Rust crate is ported from the Java class `jintspan` and the Perl module `AlignDB::IntSpan`,
//! which contains many codes from `Set::IntSpan`, `Set::IntSpan::Fast` and `Set::IntSpan::Island`.
//!

use std::cmp::{min, Ordering};
use std::collections::VecDeque;
use std::fmt;
use std::vec::Vec;

#[derive(Default, Clone)]
pub struct IntSpan {
    edges: VecDeque<i32>,
}

lazy_static! {
    static ref POS_INF: i32 = 2_147_483_647 - 1; // INT_MAX - 1, Real Largest int is POS_INF - 1
    static ref NEG_INF: i32 = -2_147_483_648 + 1;
    static ref EMPTY_STRING: String = "-".to_string();
}

//----------------------------------------------------------
/// INTERFACE: Set creation and contents
//----------------------------------------------------------
impl IntSpan {
    pub fn new() -> Self {
        IntSpan {
            edges: VecDeque::new(),
        }
    }

    pub fn from(runlist: &str) -> Self {
        let mut new = Self::new();
        new.add_runlist(runlist);

        new
    }

    pub fn from_pair(lower: i32, upper: i32) -> Self {
        let mut new = Self::new();
        new.add_pair(lower, upper);

        new
    }

    pub fn get_neg_inf(&self) -> i32 {
        *NEG_INF
    }

    pub fn get_pos_inf(&self) -> i32 {
        *POS_INF - 1
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

    pub fn to_vec(&self) -> Vec<i32> {
        let mut elements: Vec<i32> = Vec::new();

        for i in 0..self.span_size() {
            let lower = *self.edges.get(i * 2).unwrap();
            let upper = *self.edges.get(i * 2 + 1).unwrap() - 1;

            let span_len = upper - lower + 1;
            for j in 0..span_len {
                elements.push(lower + j);
            }
        }

        elements
    }

    pub fn ranges(&self) -> Vec<i32> {
        let mut ranges: Vec<i32> = Vec::new();

        for i in 0..self.edges.len() {
            // odd index means upper
            if (i & 1) == 1 {
                ranges.push(*self.edges.get(i).unwrap() - 1);
            } else {
                ranges.push(*self.edges.get(i).unwrap());
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

        *self.edges.front().unwrap()
    }

    pub fn max(&self) -> i32 {
        if self.is_empty() {
            panic!("Can't get extrema for empty IntSpan");
        }

        *self.edges.back().unwrap() - 1
    }
}

#[cfg(test)]
mod create {
    use super::*;

    #[test]
    fn test_create() {
        let tests = vec![
            ("", "-", vec![]),
            ("-", "-", vec![]),
            ("0", "0", vec![0]),
            ("1", "1", vec![1]),
            ("-1", "-1", vec![-1]),
            ("1-2", "1-2", vec![1, 2]),
            ("-2--1", "-2--1", vec![-2, -1]),
            ("-2-1", "-2-1", vec![-2, -1, 0, 1]),
            ("1,3-4", "1,3-4", vec![1, 3, 4]),
            ("1-1", "1", vec![1]),
            ("1,2-4", "1-4", vec![1, 2, 3, 4]),
            ("1-3,4", "1-4", vec![1, 2, 3, 4]),
            ("1-3,4,5-7", "1-7", vec![1, 2, 3, 4, 5, 6, 7]),
            ("1,2,3,4,5,6,7", "1-7", vec![1, 2, 3, 4, 5, 6, 7]),
        ];

        // create new
        for (runlist, exp_runlist, exp_elements) in &tests {
            let mut intspan = IntSpan::new();
            intspan.add_runlist(*runlist);

            assert_eq!(intspan.cardinality(), exp_elements.len() as i32);
            assert_eq!(intspan.size(), exp_elements.len() as i32);
            assert_eq!(intspan.to_string(), *exp_runlist);
            assert_eq!(intspan.runlist(), *exp_runlist);
            assert_eq!(intspan.to_vec(), *exp_elements);
            assert_eq!(intspan.elements(), *exp_elements);
        }

        for (runlist, exp_runlist, exp_elements) in &tests {
            let intspan = IntSpan::from(*runlist);

            assert_eq!(intspan.cardinality(), exp_elements.len() as i32);
            assert_eq!(intspan.to_string(), *exp_runlist);
            assert_eq!(intspan.to_vec(), *exp_elements);
        }

        for (_, exp_runlist, exp_elements) in &tests {
            let mut intspan = IntSpan::new();
            intspan.add_vec(exp_elements);

            assert_eq!(intspan.cardinality(), exp_elements.len() as i32);
            assert_eq!(intspan.to_string(), *exp_runlist);
            assert_eq!(intspan.to_vec(), *exp_elements);
        }
    }

    #[test]
    #[should_panic(expected = "Bad order: 1,-1")]
    fn panic_pair() {
        let mut set = IntSpan::new();
        set.add_pair(1, -1);
        println!("{:?}", set.ranges());
    }

    #[test]
    #[should_panic(expected = "Bad order: 1,-1")]
    fn panic_runlist() {
        let mut set = IntSpan::new();
        set.add_runlist("1--1");
        println!("{:?}", set.ranges());
    }

    #[test]
    #[should_panic(expected = "Number format error: a at 0 of abc")]
    fn panic_runlist_2() {
        let mut set = IntSpan::new();
        set.add_runlist("abc");
        println!("{:?}", set.ranges());
    }

    // Read as 1-11
    //#[test]
    //#[should_panic(expected = "Bad order: 1,-1")]
    //fn panic_runlist_3() {
    //    let mut set = IntSpan::new();
    //    set.add_runlist("1-1--1");
    //    println!("{:?}", set.ranges());
    //}
}

//----------------------------------------------------------
/// INTERFACE: Span contents
//----------------------------------------------------------
impl IntSpan {
    /// Returns the runs in IntSpan, as a vector of Tuple(lower, upper)
    ///
    /// ```
    /// let ints = intspan::IntSpan::from("1-2,4-7");
    /// assert_eq!(ints.spans(), vec![(1, 2), (4, 7)]);
    /// ```
    pub fn spans(&self) -> Vec<(i32, i32)> {
        let mut spans: Vec<(i32, i32)> = vec![];

        for i in 0..self.span_size() {
            let lower = *self.edges.get(i * 2).unwrap();
            let upper = *self.edges.get(i * 2 + 1).unwrap() - 1;

            spans.push((lower, upper));
        }

        spans
    }
}

#[cfg(test)]
mod content {
    use super::*;

    #[test]
    fn test_content() {
        let tests = vec![
            ("-", "-", vec![]),
            ("0", "0", vec![(0, 0)]),
            ("1", "1", vec![(1, 1)]),
            ("-1", "-1", vec![(-1, -1)]),
            ("1-2", "1-2", vec![(1, 2)]),
            ("-2--1", "-2--1", vec![(-2, -1)]),
            ("-2-1", "-2-1", vec![(-2, 1)]),
            ("1,3-4", "1,3-4", vec![(1, 1), (3, 4)]),
            ("1-2,4-7", "1-2,4-7", vec![(1, 2), (4, 7)]),
        ];

        // spans
        for (runlist, _, exp_spans) in &tests {
            let mut ints = IntSpan::new();
            ints.add_runlist(*runlist);

            let res = ints.spans();

            assert_eq!(res, *exp_spans);
        }
    }
}

//----------------------------------------------------------
/// INTERFACE: Set cardinality
//----------------------------------------------------------
impl IntSpan {
    pub fn cardinality(&self) -> i32 {
        let mut cardinality: i32 = 0;

        if self.is_empty() {
            return cardinality;
        }

        for i in 0..self.span_size() {
            let lower = *self.edges.get(i * 2).unwrap();
            let upper = *self.edges.get(i * 2 + 1).unwrap() - 1;

            cardinality += upper - lower + 1;
        }

        cardinality
    }

    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }

    pub fn is_neg_inf(&self) -> bool {
        *self.edges.front().unwrap() == *NEG_INF
    }

    pub fn is_pos_inf(&self) -> bool {
        *self.edges.back().unwrap() == *POS_INF
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
/// INTERFACE: Member operations (mutate original set)
//----------------------------------------------------------
impl IntSpan {
    pub fn add_pair(&mut self, mut lower: i32, mut upper: i32) {
        if lower > upper {
            panic!("Bad order: {},{}", lower, upper)
        }

        upper += 1;

        let mut lower_pos = self.find_pos(lower, 0);
        let mut upper_pos = self.find_pos(upper + 1, lower_pos);

        if lower_pos & 1 == 1 {
            lower_pos -= 1;
            lower = *self.edges.get(lower_pos).unwrap();
        }

        if upper_pos & 1 == 1 {
            upper = *self.edges.get(upper_pos).unwrap();
            upper_pos += 1;
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

    pub fn add_ranges(&mut self, ranges: &[i32]) {
        if ranges.len() % 2 != 0 {
            panic!("Number of ranges must be even")
        }

        for i in 0..(ranges.len() / 2) {
            let lower = *ranges.get(i * 2).unwrap();
            let upper = *ranges.get(i * 2 + 1).unwrap();

            self.add_pair(lower, upper);
        }

        // CAUTIONS: can't capture bad orders
        //        // When this IntSpan is empty, just convert ranges to edges
        //        if self.is_empty() {
        //            for i in 0..ranges.len() {
        //                // odd index means upper
        //                if (i & 1) == 1 {
        //                    self.edges.push(*ranges.get(i).unwrap() + 1);
        //                } else {
        //                    self.edges.push(*ranges.get(i).unwrap());
        //                }
        //            }
        //        } else {
        //            for i in 0..(ranges.len() / 2) {
        //                let lower = *ranges.get(i * 2).unwrap();
        //                let upper = *ranges.get(i * 2 + 1).unwrap();
        //
        //                self.add_pair(lower, upper);
        //            }
        //        }
    }

    pub fn merge(&mut self, other: &Self) {
        let ranges = other.ranges();

        self.add_ranges(&ranges);
    }

    pub fn add_vec(&mut self, ints: &[i32]) {
        let ranges = self.list_to_ranges(ints);

        self.add_ranges(&ranges);
    }

    // https://hermanradtke.com/2015/05/06/creating-a-rust-function-that-accepts-string-or-str.html
    pub fn add_runlist(&mut self, runlist: &str) {
        // skip empty runlist
        if !runlist.is_empty() && !runlist.eq(&*EMPTY_STRING) {
            let ranges = self.runlist_to_ranges(runlist);
            self.add_ranges(&ranges);
        }
    }

    pub fn invert(&mut self) {
        if self.is_empty() {
            // Universal set
            self.edges.push_back(*NEG_INF);
            self.edges.push_back(*POS_INF);
        } else {
            // Either add or remove infinity from each end. The net effect is always an even number
            // of additions and deletions

            if self.is_neg_inf() {
                self.edges.pop_front(); // shift
            } else {
                self.edges.push_front(*NEG_INF); // unshift
            }

            if self.is_pos_inf() {
                self.edges.pop_back(); // pop
            } else {
                self.edges.push_back(*POS_INF); // push
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

    pub fn remove_ranges(&mut self, ranges: &[i32]) {
        if ranges.len() % 2 != 0 {
            panic!("Number of ranges must be even");
        }

        self.invert();
        self.add_ranges(ranges);
        self.invert();
    }

    pub fn subtract(&mut self, other: &Self) {
        let ranges = other.ranges();

        self.remove_ranges(&ranges);
    }

    pub fn remove_vec(&mut self, ints: &[i32]) {
        let ranges = self.list_to_ranges(ints);

        self.remove_ranges(&ranges);
    }

    pub fn remove_runlist(&mut self, runlist: &str) {
        // skip empty runlist
        if !runlist.is_empty() && !runlist.eq(&*EMPTY_STRING) {
            let ranges = self.runlist_to_ranges(runlist);
            self.remove_ranges(&ranges);
        }
    }
}

#[cfg(test)]
mod mutate {
    use super::*;

    #[test]
    fn test_mutate() {
        let sets = vec!["-", "1", "1-2", "1,3-5"];

        let contains = vec![
            vec![false, false, false, false],
            vec![true, false, false, false],
            vec![true, true, false, false],
            vec![true, false, true, true],
        ];

        let added = vec![
            vec!["1", "2", "3", "4"],
            vec!["1", "1-2", "1,3", "1,4"],
            vec!["1-2", "1-2", "1-3", "1-2,4"],
            vec!["1,3-5", "1-5", "1,3-5", "1,3-5"],
        ];

        let removed = vec![
            vec!["-", "-", "-", "-"],
            vec!["-", "1", "1", "1"],
            vec!["2", "1", "1-2", "1-2"],
            vec!["3-5", "1,3-5", "1,4-5", "1,3,5"],
        ];

        for i in 0..4 {
            for j in 0..4 {
                let n = j + 1;

                let set = IntSpan::from(sets[i]);
                let mut set_added = set.copy();
                set_added.add_n(n);

                let mut set_removed = set.copy();
                set_removed.remove_n(n);

                // contains
                assert_eq!(set.contains(n), contains[i as usize][j as usize]);

                // added
                assert_eq!(
                    set_added.to_string(),
                    added[i as usize][j as usize].to_string()
                );

                // removed
                assert_eq!(
                    set_removed.to_string(),
                    removed[i as usize][j as usize].to_string()
                );
            }
        }
    }
}

//----------------------------------------------------------
/// INTERFACE: Set binary operations (create new set)
//----------------------------------------------------------
impl IntSpan {
    pub fn copy(&self) -> Self {
        IntSpan {
            edges: self.edges.clone(),
        }
    }

    pub fn union(&self, other: &Self) -> Self {
        let mut new = self.copy();
        new.merge(other);
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
            new.subtract(other);
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
        let mut new = self.union(other);
        let intersect = self.intersect(other);
        new.subtract(&intersect);
        new
    }
}

#[cfg(test)]
mod binary {
    use super::*;

    #[test]
    fn test_binary() {
        //   A    B    U    I    X    A-B  B-A
        let tests = vec![
            ("-", "-", "-", "-", "-", "-", "-"),
            ("1", "1", "1", "1", "-", "-", "-"),
            ("1", "2", "1-2", "-", "1-2", "1", "2"),
            ("3-9", "1-2", "1-9", "-", "1-9", "3-9", "1-2"),
            ("3-9", "1-5", "1-9", "3-5", "1-2,6-9", "6-9", "1-2"),
            ("3-9", "4-8", "3-9", "4-8", "3,9", "3,9", "-"),
            ("3-9", "5-12", "3-12", "5-9", "3-4,10-12", "3-4", "10-12"),
            ("3-9", "10-12", "3-12", "-", "3-12", "3-9", "10-12"),
            (
                "1-3,5,8-11",
                "1-6",
                "1-6,8-11",
                "1-3,5",
                "4,6,8-11",
                "8-11",
                "4,6",
            ),
        ];

        for (a, b, u, i, x, ab, ba) in tests {
            let ia = IntSpan::from(a);
            let ib = IntSpan::from(b);

            // union
            assert_eq!(ia.union(&ib).to_string(), u);

            // intersect
            assert_eq!(ia.intersect(&ib).to_string(), i);

            // xor
            assert_eq!(ia.xor(&ib).to_string(), x);

            // diff A-B
            assert_eq!(ia.diff(&ib).to_string(), ab);

            // diff B-A
            assert_eq!(ib.diff(&ia).to_string(), ba);
        }
    }
}

//----------------------------------------------------------
/// INTERFACE: Set relations
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
        self.diff(other).is_empty()
    }

    pub fn superset(&self, other: &Self) -> bool {
        other.diff(self).is_empty()
    }
}

#[cfg(test)]
mod relation {
    use super::*;

    #[test]
    fn test_relation() {
        let sets = vec!["-", "1", "5", "1-5", "3-7", "1-3,8,10-23"];

        let equals = vec![
            vec![1, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 1, 0],
            vec![0, 0, 0, 0, 0, 1],
        ];

        let subset = vec![
            vec![1, 1, 1, 1, 1, 1],
            vec![0, 1, 0, 1, 0, 1],
            vec![0, 0, 1, 1, 1, 0],
            vec![0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 1, 0],
            vec![0, 0, 0, 0, 0, 1],
        ];

        let superset = vec![
            vec![1, 0, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 0, 0],
            vec![1, 0, 1, 0, 0, 0],
            vec![1, 1, 1, 1, 0, 0],
            vec![1, 0, 1, 0, 1, 0],
            vec![1, 1, 0, 0, 0, 1],
        ];

        for i in 0..6 {
            for j in 0..6 {
                let a = IntSpan::from(sets[i]);
                let b = IntSpan::from(sets[j]);

                // equals
                assert_eq!(a.equals(&b), equals[i as usize][j as usize] != 0);

                // subset
                assert_eq!(a.subset(&b), subset[i as usize][j as usize] != 0);

                // superset
                assert_eq!(a.superset(&b), superset[i as usize][j as usize] != 0);
            }
        }
    }
}

//----------------------------------------------------------
/// INTERFACE: Indexing
//----------------------------------------------------------
impl IntSpan {
    fn at_pos(&self, index: i32) -> i32 {
        let mut element = self.min();
        let mut ele_before = 0;

        for i in 0..self.span_size() {
            let lower = *self.edges.get(i * 2).unwrap();
            let upper = *self.edges.get(i * 2 + 1).unwrap() - 1;

            let span_len = upper - lower + 1;

            if index > ele_before + span_len {
                ele_before += span_len;
            } else {
                element = index - ele_before - 1 + lower;
                break;
            }
        }

        element
    }

    fn at_neg(&self, index: i32) -> i32 {
        let mut element = self.max();
        let mut ele_after = 0;

        for i in (0..self.span_size()).rev() {
            let lower = *self.edges.get(i * 2).unwrap();
            let upper = *self.edges.get(i * 2 + 1).unwrap() - 1;

            let span_len = upper - lower + 1;

            if index > ele_after + span_len {
                ele_after += span_len;
            } else {
                element = upper - (index - ele_after) + 1;
                break;
            }
        }

        element
    }

    /// Returns the index-th element of set, indices start from `1`.
    ///
    /// Negative indices count backwards from the end of the set.
    pub fn at(&self, index: i32) -> i32 {
        if self.is_empty() {
            panic!("Indexing on an empty set");
        }
        if i32::abs(index) < 1 {
            panic!("Index can't be 0");
        }
        if i32::abs(index) > self.cardinality() {
            panic!("Out of max index");
        }

        if index > 0 {
            self.at_pos(index)
        } else {
            self.at_neg(-index)
        }
    }

    /// Returns the index of an element in the set, indices start from `1`
    pub fn index(&self, element: i32) -> i32 {
        if self.is_empty() {
            panic!("Indexing on an empty set");
        }
        if !self.contains(element) {
            panic!("Element doesn't exist");
        }

        let mut index = -1; // not valid
        let mut ele_before = 0;

        for i in 0..self.span_size() {
            let lower = *self.edges.get(i * 2).unwrap();
            let upper = *self.edges.get(i * 2 + 1).unwrap() - 1;
            let span_len = upper - lower + 1;

            if element >= lower && element <= upper {
                index = element - lower + 1 + ele_before;
            } else {
                ele_before += span_len;
            }
        }

        index
    }

    pub fn slice(&self, from: i32, to: i32) -> IntSpan {
        if self.is_empty() {
            panic!("Indexing on an empty set");
        }
        if from < 1 {
            panic!("Index can't be 0 or negative");
        }
        if to > self.cardinality() {
            panic!("Out of max index");
        }
        if from > to {
            panic!("Bad order: {},{}", from, to)
        }

        let lower = self.at(from);
        let upper = self.at(to);

        let new = IntSpan::from_pair(lower, upper);
        new.intersect(self)
    }
}

#[cfg(test)]
mod index {
    use super::*;

    #[test]
    fn test_index() {
        // runlist, n, exp_index, exp_element
        let tests = vec![
            // None
            ("-", 1, None, None),
            ("-", -1, None, None),
            ("1-10,21-30", 25, None, Some(15)),
            ("1-10,21-30", -25, None, None),
            // at_pos
            ("0-9", 1, Some(0), Some(2)),
            ("0-9", 6, Some(5), Some(7)),
            ("0-9", 10, Some(9), None),
            ("0-9", 11, None, None),
            // at_neg
            ("0-9", -1, Some(9), None),
            ("0-9", -5, Some(5), None),
            ("0-9", -10, Some(0), None),
            ("0-9", -11, None, None),
            // at_pos
            ("1-10,21-30,41-50", 6, Some(6), Some(6)),
            ("1-10,21-30,41-50", 16, Some(26), None),
            ("1-10,21-30,41-50", 26, Some(46), Some(16)),
            ("1-10,21-30,41-50", 31, None, None),
            // at_neg
            ("1-10,21-30,41-50", -1, Some(50), None),
            ("1-10,21-30,41-50", -11, Some(30), None),
            ("1-10,21-30,41-50", -21, Some(10), None),
            ("1-10,21-30,41-50", -30, Some(1), None),
            ("1-10,21-30,41-50", -31, None, None),
        ];

        for (runlist, n, exp_index, exp_element) in tests {
            let set = IntSpan::from(runlist);

            // at
            if exp_index.is_some() {
                assert_eq!(set.at(n), exp_index.unwrap());
            }

            // index
            if exp_element.is_some() {
                assert_eq!(set.index(n), exp_element.unwrap());
            }
        }
    }

    #[test]
    fn test_slice() {
        // runlist, from, to, exp
        let tests = vec![
            ("1-10,21-30,41-50", 1, 3, "1-3"),
            ("1-10,21-30,41-50", 6, 8, "6-8"),
            ("1-10,21-30,41-50", 8, 10, "8-10"),
            ("1-10,21-30,41-50", 10, 10, "10"),
        ];

        for (runlist, from, to, exp) in tests {
            let set = IntSpan::from(runlist);

            assert_eq!(set.slice(from, to).to_string(), exp);
        }
    }

    #[test]
    #[should_panic(expected = "Indexing on an empty set")]
    fn panic_at_1() {
        let set = IntSpan::new();
        set.at(1);
        println!("{:?}", set.ranges());
    }

    #[test]
    #[should_panic(expected = "Index can't be 0")]
    fn panic_at_2() {
        let set = IntSpan::from("0-9");
        set.at(0);
        println!("{:?}", set.ranges());
    }

    #[test]
    #[should_panic(expected = "Out of max index")]
    fn panic_at_3() {
        let set = IntSpan::from("0-9");
        set.at(15);
        println!("{:?}", set.ranges());
    }

    #[test]
    #[should_panic(expected = "Indexing on an empty set")]
    fn panic_index_1() {
        let set = IntSpan::new();
        set.index(1);
        println!("{:?}", set.ranges());
    }

    #[test]
    #[should_panic(expected = "Element doesn't exist")]
    fn panic_index_2() {
        let set = IntSpan::from("0-9");
        set.index(15);
        println!("{:?}", set.ranges());
    }

    #[test]
    #[should_panic(expected = "Indexing on an empty set")]
    fn panic_slice_1() {
        let set = IntSpan::new();
        set.slice(1, 2);
        println!("{:?}", set.ranges());
    }
}

//----------------------------------------------------------
/// INTERFACE: Spans Ops
//----------------------------------------------------------
impl IntSpan {
    pub fn cover(&self) -> Self {
        let mut new = IntSpan::new();
        if !self.is_empty() {
            new.add_pair(self.min(), self.max());
        }
        new
    }

    pub fn holes(&self) -> Self {
        let mut new = IntSpan::new();
        if self.is_empty() || self.is_universal() {
            // empty and universal set have no holes
            return new;
        }
        let complement = self.complement();
        let mut ranges = complement.ranges();

        // Remove infinite arms of complement set
        if complement.is_neg_inf() {
            ranges.remove(0);
            ranges.remove(0);
        }
        if complement.is_pos_inf() {
            ranges.pop();
            ranges.pop();
        }

        new.add_ranges(&ranges);

        new
    }

    pub fn inset(&self, n: i32) -> Self {
        let mut new = IntSpan::new();

        for i in 0..self.span_size() {
            let mut lower = *self.edges.get(i * 2).unwrap();
            let mut upper = *self.edges.get(i * 2 + 1).unwrap() - 1;

            if lower != self.get_neg_inf() {
                lower += n;
            }
            if upper != self.get_pos_inf() {
                upper -= n;
            }

            if lower <= upper {
                new.add_pair(lower, upper);
            }
        }

        new
    }

    pub fn trim(&self, n: i32) -> Self {
        self.inset(n)
    }

    pub fn pad(&self, n: i32) -> Self {
        self.inset(-n)
    }

    pub fn excise(&self, min_len: i32) -> Self {
        let mut new = IntSpan::new();

        for i in 0..self.span_size() {
            let lower = *self.edges.get(i * 2).unwrap();
            let upper = *self.edges.get(i * 2 + 1).unwrap() - 1;

            let span_len = upper - lower + 1;
            if span_len >= min_len {
                new.add_pair(lower, upper);
            }
        }

        new
    }

    pub fn fill(&self, max_len: i32) -> Self {
        let mut new = self.copy();
        let holes = self.holes();

        for i in 0..holes.span_size() {
            let lower = *holes.edges.get(i * 2).unwrap();
            let upper = *holes.edges.get(i * 2 + 1).unwrap() - 1;

            let span_len = upper - lower + 1;
            if span_len <= max_len {
                new.add_pair(lower, upper);
            }
        }

        new
    }
}

#[cfg(test)]
mod span {
    use super::*;

    #[test]
    fn cover_holes() {
        // runlist expCover expHoles
        let tests = vec![
            ("-", "-", "-"),
            ("1", "1", "-"),
            ("5", "5", "-"),
            ("1,3,5", "1-5", "2,4"),
            ("1,3-5", "1-5", "2"),
            ("1-3,5,8-11", "1-11", "4,6-7"),
        ];

        for (runlist, exp_cover, exp_holes) in tests {
            let set = IntSpan::from(runlist);

            // cover
            assert_eq!(set.cover().to_string(), exp_cover);

            // holes
            assert_eq!(set.holes().to_string(), exp_holes);
        }
    }

    #[test]
    fn inset() {
        let neg = IntSpan::new().get_neg_inf();
        let pos = IntSpan::new().get_pos_inf();

        let uni = format!("{}-{}", neg, pos);

        // runlist n expected
        let tests = vec![
            ("-".to_string(), -2, "-".to_string()),
            ("-".to_string(), -1, "-".to_string()),
            ("-".to_string(), 0, "-".to_string()),
            ("-".to_string(), 1, "-".to_string()),
            ("-".to_string(), 2, "-".to_string()),
            (uni.clone(), -2, uni.clone()),
            (uni.clone(), 2, uni.clone()),
            (format!("{}-0", neg), -2, format!("{}-2", neg)),
            (format!("{}-0", neg), 2, format!("{}--2", neg)),
            (format!("0-{}", pos), -2, format!("-2-{}", pos)),
            (format!("0-{}", pos), 2, format!("2-{}", pos)),
            (
                "0,2-3,6-8,12-15,20-24,30-35".to_string(),
                -2,
                "-2-26,28-37".to_string(),
            ),
            (
                "0,2-3,6-8,12-15,20-24,30-35".to_string(),
                -1,
                "-1-9,11-16,19-25,29-36".to_string(),
            ),
            (
                "0,2-3,6-8,12-15,20-24,30-35".to_string(),
                0,
                "0,2-3,6-8,12-15,20-24,30-35".to_string(),
            ),
            (
                "0,2-3,6-8,12-15,20-24,30-35".to_string(),
                1,
                "7,13-14,21-23,31-34".to_string(),
            ),
            (
                "0,2-3,6-8,12-15,20-24,30-35".to_string(),
                2,
                "22,32-33".to_string(),
            ),
        ];

        // inset
        for (runlist, n, expected) in tests {
            let set = IntSpan::from(&runlist);
            assert_eq!(set.inset(n).to_string(), expected);
        }

        // trim and pad
        assert_eq!(IntSpan::from("1-3").pad(1).cardinality(), 5);
        assert_eq!(IntSpan::from("1-3").pad(2).cardinality(), 7);
        assert_eq!(IntSpan::from("1-3").trim(1).cardinality(), 1);
        assert_eq!(IntSpan::from("1-3").trim(2).cardinality(), 0);
    }

    #[test]
    fn excise_fill() {
        // runlist n expExcise expFill
        let tests = vec![
            ("1-5", 1, "1-5", "1-5"),
            ("1-5,7", 1, "1-5,7", "1-7"),
            ("1-5,7", 2, "1-5", "1-7"),
            ("1-5,7-8", 1, "1-5,7-8", "1-8"),
            ("1-5,7-8", 3, "1-5", "1-8"),
            ("1-5,7-8", 6, "-", "1-8"),
            ("1-5,7,9-10", 0, "1-5,7,9-10", "1-5,7,9-10"),
            ("1-5,9-10", 2, "1-5,9-10", "1-5,9-10"),
            ("1-5,9-10", 3, "1-5", "1-10"),
            ("1-5,9-10,12-13,15", 2, "1-5,9-10,12-13", "1-5,9-15"),
            ("1-5,9-10,12-13,15", 3, "1-5", "1-15"),
        ];

        for (runlist, n, exp_excise, exp_fill) in tests {
            let set = IntSpan::from(runlist);

            // excise
            assert_eq!(set.excise(n).to_string(), exp_excise);

            // fill
            assert_eq!(set.fill(n).to_string(), exp_fill);
        }
    }
}

//----------------------------------------------------------
/// INTERFACE: Inter-set OPs
//----------------------------------------------------------
impl IntSpan {
    /// `overlap`
    ///
    /// Returns the size of intersection of two sets.
    ///
    /// `set.overlap(&other)` equivalent to `set.intersect(&other).cardinality()`
    ///
    /// ```
    /// # use intspan::IntSpan;
    /// let set = IntSpan::from("1");
    /// let other = IntSpan::from("1");
    /// assert_eq!(set.overlap(&other), 1);
    /// let other = IntSpan::from("2");
    /// assert_eq!(set.overlap(&other), 0);
    /// let set = IntSpan::from("1-5");
    /// let other = IntSpan::from("1-10");
    /// assert_eq!(set.overlap(&other), 5);
    /// let set = IntSpan::from("1-5,6");
    /// let other = IntSpan::from("6-10");
    /// assert_eq!(set.overlap(&other), 1);
    /// ```
    pub fn overlap(&self, other: &Self) -> i32 {
        self.intersect(other).cardinality()
    }

    /// Returns the distance between sets, measured as follows.
    ///
    /// * If the sets overlap, then the distance is negative and given by `- set.overlap(&other)`
    ///
    /// * If the sets do not overlap, $d is positive and given by the distance on the integer line
    ///   between the two closest islands of the sets.
    ///
    /// ```
    /// # use intspan::IntSpan;
    /// let set = IntSpan::from("1");
    /// let other = IntSpan::from("1");
    /// assert_eq!(set.distance(&other), -1);
    /// let other = IntSpan::from("");
    /// assert_eq!(set.distance(&other), 0);
    /// let other = IntSpan::from("2");
    /// assert_eq!(set.distance(&other), 1);
    ///
    /// let set = IntSpan::from("1-5");
    /// let other = IntSpan::from("1-10");
    /// assert_eq!(set.distance(&other), -5);
    /// let other = IntSpan::from("10-15");
    /// assert_eq!(set.distance(&other), 5);
    /// let set = IntSpan::from("1-5,6");
    /// let other = IntSpan::from("6-10");
    /// assert_eq!(set.distance(&other), -1);
    ///
    /// let set = IntSpan::from("1-5,10-15");
    /// let other = IntSpan::from("5-9");
    /// assert_eq!(set.distance(&other), -1);
    /// let other = IntSpan::from("6");
    /// assert_eq!(set.distance(&other), 1);
    /// let other = IntSpan::from("7");
    /// assert_eq!(set.distance(&other), 2);
    /// let other = IntSpan::from("7-9");
    /// assert_eq!(set.distance(&other), 1);
    /// let other = IntSpan::from("16-20");
    /// assert_eq!(set.distance(&other), 1);
    /// let other = IntSpan::from("17-20");
    /// assert_eq!(set.distance(&other), 2);
    /// ```
    pub fn distance(&self, other: &Self) -> i32 {
        if self.is_empty() || other.is_empty() {
            0
        } else {
            let overlap = self.overlap(other);

            if overlap > 0 {
                -overlap
            } else {
                let mut min_d = 0;

                for i in 0..self.span_size() {
                    let lower1 = *self.edges.get(i * 2).unwrap();
                    let upper1 = *self.edges.get(i * 2 + 1).unwrap() - 1;
                    for j in 0..other.span_size() {
                        let lower2 = *other.edges.get(j * 2).unwrap();
                        let upper2 = *other.edges.get(j * 2 + 1).unwrap() - 1;

                        let d1 = (lower1 - upper2).abs();
                        let d2 = (upper1 - lower2).abs();
                        let d = min(d1, d2);

                        if min_d == 0 || d < min_d {
                            min_d = d;
                        }
                    }
                }
                min_d
            }
        }
    }
}

//----------------------------------------------------------
// TODO: Islands
//----------------------------------------------------------

//----------------------------------------------------------
/// INTERFACE: Aliases
//----------------------------------------------------------
impl IntSpan {
    pub fn size(&self) -> i32 {
        self.cardinality()
    }

    pub fn runlist(&self) -> String {
        self.to_string()
    }

    pub fn elements(&self) -> Vec<i32> {
        self.to_vec()
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
            let mid_edge = self.edges.get(mid).unwrap();
            match val.cmp(mid_edge) {
                Ordering::Less => high = mid,
                Ordering::Greater => low = mid + 1,
                Ordering::Equal => return mid,
            }
        }

        low
    }

    fn list_to_ranges(&self, ints: &[i32]) -> Vec<i32> {
        let mut ranges: Vec<i32> = Vec::new();

        let mut ints = ints.to_owned();
        ints.sort_unstable();
        ints.dedup();

        let len = ints.len();
        let mut pos: usize = 0;

        while pos < len {
            let mut end = pos + 1;
            while (end < len) && (ints[end] <= ints[end - 1] + 1) {
                end += 1;
            }
            ranges.push(ints[pos]);
            ranges.push(ints[end - 1]);
            pos = end;
        }

        ranges
    }

    fn runlist_to_ranges(&self, runlist: &str) -> Vec<i32> {
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
            if *bytes.get(idx).unwrap() == b'-' {
                lower_is_neg = true;
                i += 1;
            }

            // ported from Java Integer.parseInt()
            let mut lower: i32 = 0;
            let mut upper: i32 = 0;

            while idx + i < len {
                let ch = bytes[idx + i];
                if ch.is_ascii_digit() {
                    if !in_upper {
                        lower *= radix;
                        lower -= (ch as char).to_digit(10).unwrap() as i32;
                    } else {
                        upper *= radix;
                        upper -= (ch as char).to_digit(10).unwrap() as i32;
                    }
                } else if ch == b'-' {
                    if !in_upper {
                        in_upper = true;
                        if *bytes.get(idx + i + 1).unwrap() == b'-' {
                            upper_is_neg = true;
                        }
                    }
                } else if ch == b',' {
                    i += 1;
                    break; // end of run
                } else {
                    panic!(
                        "Number format error: {} at {} of {}",
                        ch as char,
                        idx + i,
                        runlist
                    );
                }

                i += 1;
            }

            if !in_upper {
                ranges.push(if lower_is_neg { lower } else { -lower }); // add lower
                ranges.push(if lower_is_neg { lower } else { -lower }); // add lower again
            } else {
                ranges.push(if lower_is_neg { lower } else { -lower }); // add lower
                ranges.push(if upper_is_neg { upper } else { -upper }); // add upper
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
        if self.is_empty() {
            write!(f, "{}", *EMPTY_STRING)?;
        } else {
            let mut runlist = "".to_string();

            for i in 0..self.span_size() {
                let lower = *self.edges.get(i * 2).unwrap();
                let upper = *self.edges.get(i * 2 + 1).unwrap() - 1;

                let mut buf = "".to_string();
                if i != 0 {
                    buf.push(',');
                }

                if lower == upper {
                    buf.push_str(lower.to_string().as_str());
                } else {
                    buf.push_str(format!("{}-{}", lower, upper).as_str());
                }

                runlist.push_str(buf.as_str());
            }

            write!(f, "{}", runlist)?;
        }

        Ok(())
    }
}
