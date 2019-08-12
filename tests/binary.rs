use intspan::IntSpan;

//              A       B       U       I       X       A-B     B-A
//              0       1       2       3       4       5       6
struct TestData(String, String, String, String, String, String, String);

#[test]
fn binary() {
    let tests = vec![
        TestData(
            "-".to_string(),
            "-".to_string(),
            "-".to_string(),
            "-".to_string(),
            "-".to_string(),
            "-".to_string(),
            "-".to_string(),
        ),
        TestData(
            "1".to_string(),
            "1".to_string(),
            "1".to_string(),
            "1".to_string(),
            "-".to_string(),
            "-".to_string(),
            "-".to_string(),
        ),
        TestData(
            "1".to_string(),
            "2".to_string(),
            "1-2".to_string(),
            "-".to_string(),
            "1-2".to_string(),
            "1".to_string(),
            "2".to_string(),
        ),
        TestData(
            "3-9".to_string(),
            "1-2".to_string(),
            "1-9".to_string(),
            "-".to_string(),
            "1-9".to_string(),
            "3-9".to_string(),
            "1-2".to_string(),
        ),
        TestData(
            "3-9".to_string(),
            "1-5".to_string(),
            "1-9".to_string(),
            "3-5".to_string(),
            "1-2,6-9".to_string(),
            "6-9".to_string(),
            "1-2".to_string(),
        ),
        TestData(
            "3-9".to_string(),
            "4-8".to_string(),
            "3-9".to_string(),
            "4-8".to_string(),
            "3,9".to_string(),
            "3,9".to_string(),
            "-".to_string(),
        ),
        TestData(
            "3-9".to_string(),
            "5-12".to_string(),
            "3-12".to_string(),
            "5-9".to_string(),
            "3-4,10-12".to_string(),
            "3-4".to_string(),
            "10-12".to_string(),
        ),
        TestData(
            "3-9".to_string(),
            "10-12".to_string(),
            "3-12".to_string(),
            "-".to_string(),
            "3-12".to_string(),
            "3-9".to_string(),
            "10-12".to_string(),
        ),
        TestData(
            "1-3,5,8-11".to_string(),
            "1-6".to_string(),
            "1-6,8-11".to_string(),
            "1-3,5".to_string(),
            "4,6,8-11".to_string(),
            "8-11".to_string(),
            "4,6".to_string(),
        ),
   ];

    for t in tests.iter() {
        let a = IntSpan::from(&t.0);
        let b = IntSpan::from(&t.1);

        // union
        assert_eq!(a.union(&b).to_string(), t.2);

        // intersect
        assert_eq!(a.intersect(&b).to_string(), t.3);

        // xor
        assert_eq!(a.xor(&b).to_string(), t.4);

        // diff A-B
        assert_eq!(a.diff(&b).to_string(), t.5);

        // diff B-A
        assert_eq!(b.diff(&a).to_string(), t.6);
    }
}
