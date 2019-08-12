use intspan::IntSpan;

struct TestData(String, String, Vec<i32>);

#[test]
fn create() {
    let tests = vec![
        TestData("".to_string(), "-".to_string(), vec![]),
        TestData("-".to_string(), "-".to_string(), vec![]),
        TestData("0".to_string(), "0".to_string(), vec![0]),
        TestData("1".to_string(), "1".to_string(), vec![1]),
        TestData("-1".to_string(), "-1".to_string(), vec![-1]),
        TestData("1-2".to_string(), "1-2".to_string(), vec![1, 2]),
        TestData("-2--1".to_string(), "-2--1".to_string(), vec![-2, -1]),
        TestData("-2-1".to_string(), "-2-1".to_string(), vec![-2, -1, 0, 1]),
        TestData("1,3-4".to_string(), "1,3-4".to_string(), vec![1, 3, 4]),
        //        TestData("1-1".to_string(), "1".to_string(), vec![1]),
        //        TestData("1,2-4".to_string(), "1-4".to_string(), vec![1,2,3,4]),
    ];

    fn create_new(tests: &Vec<TestData>) {
        for t in tests.iter() {
            let mut set = IntSpan::new();
            set.add_runlist(&t.0);

            assert_eq!(set.cardinality(), t.2.len() as i32);
            assert_eq!(set.to_string(), t.1);
            assert_eq!(set.to_vec(), t.2);
        }
    }

    fn create_from(tests: &Vec<TestData>) {
        for t in tests.iter() {
            let set = IntSpan::from(&t.0);

            assert_eq!(set.cardinality(), t.2.len() as i32);
            assert_eq!(set.to_string(), t.1);
            assert_eq!(set.to_vec(), t.2);
        }
    }

    fn create_ints(tests: &Vec<TestData>) {
        for t in tests.iter() {
            let mut set = IntSpan::new();
            set.add_vec(&t.2);

            assert_eq!(set.cardinality(), t.2.len() as i32);
            assert_eq!(set.to_string(), t.1);
            assert_eq!(set.to_vec(), t.2);
        }
    }

    create_new(&tests);
    create_from(&tests);
    create_ints(&tests);

    // TODO: failed tests
}
