use intspan::IntSpan;

#[test]
fn cover_holes() {
    // runlist expCover expHoles
    struct TestData(String, String, String);

    let tests = vec![
        TestData("-".to_string(), "-".to_string(), "-".to_string()),
        TestData("1".to_string(), "1".to_string(), "-".to_string()),
        TestData("5".to_string(), "5".to_string(), "-".to_string()),
        TestData("1,3,5".to_string(), "1-5".to_string(), "2,4".to_string()),
        TestData("1,3-5".to_string(), "1-5".to_string(), "2".to_string()),
        TestData(
            "1-3,5,8-11".to_string(),
            "1-11".to_string(),
            "4,6-7".to_string(),
        ),
    ];

    for t in tests.iter() {
        let set = IntSpan::from(&t.0);

        // cover
        assert_eq!(set.cover().to_string(), t.1);

        // holes
        assert_eq!(set.holes().to_string(), t.2);
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
    struct TestData(String, i32, String, String);

    let tests = vec![
        TestData("1-5".to_string(), 1, "1-5".to_string(), "1-5".to_string()),
        TestData(
            "1-5,7".to_string(),
            1,
            "1-5,7".to_string(),
            "1-7".to_string(),
        ),
        TestData("1-5,7".to_string(), 2, "1-5".to_string(), "1-7".to_string()),
        TestData(
            "1-5,7-8".to_string(),
            1,
            "1-5,7-8".to_string(),
            "1-8".to_string(),
        ),
        TestData(
            "1-5,7-8".to_string(),
            3,
            "1-5".to_string(),
            "1-8".to_string(),
        ),
        TestData("1-5,7-8".to_string(), 6, "-".to_string(), "1-8".to_string()),
        TestData(
            "1-5,7,9-10".to_string(),
            0,
            "1-5,7,9-10".to_string(),
            "1-5,7,9-10".to_string(),
        ),
        TestData(
            "1-5,9-10".to_string(),
            2,
            "1-5,9-10".to_string(),
            "1-5,9-10".to_string(),
        ),
        TestData(
            "1-5,9-10".to_string(),
            3,
            "1-5".to_string(),
            "1-10".to_string(),
        ),
        TestData(
            "1-5,9-10,12-13,15".to_string(),
            2,
            "1-5,9-10,12-13".to_string(),
            "1-5,9-15".to_string(),
        ),
        TestData(
            "1-5,9-10,12-13,15".to_string(),
            3,
            "1-5".to_string(),
            "1-15".to_string(),
        ),
    ];

    for t in tests.iter() {
        let set = IntSpan::from(&t.0);

        // excise
        assert_eq!(set.excise(t.1).to_string(), t.2);

        // fill
        assert_eq!(set.fill(t.1).to_string(), t.3);
    }
}
