
#[cfg(test)]
mod membership {
    use intspan::IntSpan;

    #[test]
    fn membership() {
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

#[cfg(test)]
mod index {
    use intspan::IntSpan;

    #[test]
    fn index() {
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
}
