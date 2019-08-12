use intspan::IntSpan;

#[test]
fn span_ops() {
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
