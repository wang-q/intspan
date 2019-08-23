mod create {
    use intspan::IntSpan;

    #[test]
    fn create() {
        let tests = vec![
            ("", "-", vec![]),
            ("-", "-", vec![]),
            ("0", "0", vec![0]),
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

mod span {
    use intspan::IntSpan;

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
        // runlist n expected
        struct TestData(String, i32, String);

        let universal = format!(
            "{}-{}",
            IntSpan::new().get_neg_inf(),
            IntSpan::new().get_pos_inf()
        );

        let tests = vec![
            TestData("-".to_string(), -2, "-".to_string()),
            TestData("-".to_string(), -1, "-".to_string()),
            TestData("-".to_string(), 0, "-".to_string()),
            TestData("-".to_string(), 1, "-".to_string()),
            TestData("-".to_string(), 2, "-".to_string()),
            TestData(universal.clone(), -2, universal.clone()),
            TestData(universal.clone(), 2, universal.clone()),
            TestData(
                format!("{}-0", IntSpan::new().get_neg_inf()),
                -2,
                format!("{}-2", IntSpan::new().get_neg_inf()),
            ),
            TestData(
                format!("{}-0", IntSpan::new().get_neg_inf()),
                2,
                format!("{}--2", IntSpan::new().get_neg_inf()),
            ),
            TestData(
                format!("0-{}", IntSpan::new().get_pos_inf()),
                -2,
                format!("-2-{}", IntSpan::new().get_pos_inf()),
            ),
            TestData(
                format!("0-{}", IntSpan::new().get_pos_inf()),
                2,
                format!("2-{}", IntSpan::new().get_pos_inf()),
            ),
            TestData(
                "0,2-3,6-8,12-15,20-24,30-35".to_string(),
                -2,
                "-2-26,28-37".to_string(),
            ),
            TestData(
                "0,2-3,6-8,12-15,20-24,30-35".to_string(),
                -1,
                "-1-9,11-16,19-25,29-36".to_string(),
            ),
            TestData(
                "0,2-3,6-8,12-15,20-24,30-35".to_string(),
                0,
                "0,2-3,6-8,12-15,20-24,30-35".to_string(),
            ),
            TestData(
                "0,2-3,6-8,12-15,20-24,30-35".to_string(),
                1,
                "7,13-14,21-23,31-34".to_string(),
            ),
            TestData(
                "0,2-3,6-8,12-15,20-24,30-35".to_string(),
                2,
                "22,32-33".to_string(),
            ),
        ];

        // inset
        for t in tests.iter() {
            let set = IntSpan::from(&t.0);
            assert_eq!(set.inset(t.1).to_string(), t.2);
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

mod relation {
    use intspan::IntSpan;

    #[test]
    fn relation() {
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
