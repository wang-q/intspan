use crate::IntSpan;
use std::collections::BTreeMap;

#[derive(Default, Clone)]
pub struct Coverage {
    max: i32,
    tiers: BTreeMap<i32, IntSpan>,
}

impl Coverage {
    pub fn max(&self) -> &i32 {
        &self.max
    }
    pub fn tiers(&self) -> &BTreeMap<i32, IntSpan> {
        &self.tiers
    }

    pub fn new(max: i32) -> Self {
        Self::new_len(max, 1_000_000_000)
    }

    pub fn new_len(max: i32, len: i32) -> Self {
        let mut tiers: BTreeMap<i32, IntSpan> = BTreeMap::new();
        tiers.insert(-1, IntSpan::from_pair(1, len));
        tiers.insert(0, IntSpan::from_pair(1, len));

        for i in 1..=max {
            tiers.insert(i, IntSpan::new());
        }

        Self { max, tiers }
    }

    fn begin_end(begin: i32, end: i32) -> (i32, i32) {
        let mut tup = (begin.min(end), begin.max(end));

        if tup.0 == 0 {
            tup.0 = 1;
        }

        tup
    }

    /// ```
    /// # use intspan::Coverage;
    /// let mut cover = Coverage::new(1);
    /// cover.bump(1, 100);
    /// cover.bump(90, 150);
    /// assert_eq!(cover.tiers().get(&1).unwrap().to_string(), "1-150");
    /// # assert_eq!(cover.tiers().get(&0).unwrap().to_string(), "151-1000000000");
    ///
    /// let mut cover = Coverage::new_len(1, 500);
    /// cover.bump(1, 100);
    /// cover.bump(90, 150);
    /// assert_eq!(cover.tiers().get(&1).unwrap().to_string(), "1-150");
    /// # assert_eq!(cover.tiers().get(&0).unwrap().to_string(), "151-500");
    /// # assert_eq!(cover.tiers().get(&-1).unwrap().to_string(), "1-500");
    /// ```
    pub fn bump(&mut self, begin: i32, end: i32) {
        let tup = Self::begin_end(begin, end);
        let mut intspan = IntSpan::from_pair(tup.0, tup.1);

        // reach max coverage in full sequence
        if self
            .tiers
            .get(&-1)
            .unwrap()
            .equals(self.tiers.get(&self.max).unwrap())
        {
            return;
        }

        // remove intspan from uncovered regions
        self.tiers.entry(0).and_modify(|e| e.subtract(&intspan));

        for i in 1..=self.max {
            let intersect = self.tiers.get(&i).unwrap().intersect(&intspan);
            self.tiers.entry(i).and_modify(|e| e.merge(&intspan));

            if i + 1 > self.max {
                break;
            }

            intspan = intersect.copy();
        }
    }

    /// ```
    /// # use intspan::Coverage;
    /// let mut cover = Coverage::new(2);
    /// cover.bump(1, 100);
    /// cover.bump(90, 150);
    /// assert_eq!(cover.max_tier().to_string(), "90-100");
    ///
    /// let mut cover = Coverage::new(5);
    /// cover.bump(1, 100);
    /// cover.bump(90, 150);
    /// assert_eq!(cover.max_tier().to_string(), "-");
    /// ```
    pub fn max_tier(&self) -> IntSpan {
        self.tiers().get(self.max()).unwrap().copy()
    }

    /// ```
    /// # use intspan::Coverage;
    /// let mut cover = Coverage::new(2);
    /// cover.bump(1, 100);
    /// cover.bump(90, 150);
    ///
    /// assert_eq!(cover.uniq_tiers().get(&2).unwrap().to_string(), "90-100");
    ///
    /// assert_eq!(cover.tiers().get(&1).unwrap().to_string(), "1-150");
    /// assert_eq!(cover.uniq_tiers().get(&1).unwrap().to_string(), "1-89,101-150");
    /// ```
    pub fn uniq_tiers(&self) -> BTreeMap<i32, IntSpan> {
        let mut tiers = self.tiers.clone();

        for i in 1..self.max {
            let intspan_next = tiers[&(i + 1)].copy();
            tiers.entry(i).and_modify(|e| e.subtract(&intspan_next));
        }

        tiers
    }
}
