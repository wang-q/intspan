use crate::IntSpan;
use std::collections::BTreeMap;

#[derive(Default)]
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
        let mut tiers: BTreeMap<i32, IntSpan> = BTreeMap::new();
        tiers.insert(-1, IntSpan::from("1-1000000000"));
        tiers.insert(0, IntSpan::from("1-1000000000"));

        for i in 1..=max {
            tiers.insert(i, IntSpan::new());
        }

        Self { max, tiers }
    }

    /// ```
    /// # use intspan::Coverage;
    /// let mut cover = Coverage::new(1);
    /// cover.bump(1, 100);
    /// cover.bump(90, 150);
    /// assert_eq!(cover.tiers().get(&1).unwrap().to_string(), "1-150");
    /// # assert_eq!(cover.tiers().get(&0).unwrap().to_string(), "151-1000000000");
    /// ```
    pub fn bump(&mut self, start: i32, end: i32) {
        let mut intspan = IntSpan::new();
        intspan.add_pair(start, end);

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
}
