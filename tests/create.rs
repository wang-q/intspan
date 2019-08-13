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
        TestData("1-1".to_string(), "1".to_string(), vec![1]),
        TestData("1,2-4".to_string(), "1-4".to_string(), vec![1, 2, 3, 4]),
        TestData("1-3,4".to_string(), "1-4".to_string(), vec![1, 2, 3, 4]),
        TestData(
            "1-3,4,5-7".to_string(),
            "1-7".to_string(),
            vec![1, 2, 3, 4, 5, 6, 7],
        ),
        TestData(
            "1,2,3,4,5,6,7".to_string(),
            "1-7".to_string(),
            vec![1, 2, 3, 4, 5, 6, 7],
        ),
    ];

    fn create_new(tests: &Vec<TestData>) {
        for t in tests.iter() {
            let mut set = IntSpan::new();
            set.add_runlist(&t.0);

            assert_eq!(set.cardinality(), t.2.len() as i32);
            assert_eq!(set.size(), t.2.len() as i32);
            assert_eq!(set.to_string(), t.1);
            assert_eq!(set.runlist(), t.1);
            assert_eq!(set.to_vec(), t.2);
            assert_eq!(set.elements(), t.2);
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
