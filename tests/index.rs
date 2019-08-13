use intspan::IntSpan;

#[test]
fn index() {
    // runlist n expIndex expElement
    struct TestData(String, i32, Option<i32>, Option<i32>);

    let tests = vec![
        // None
        TestData("-".to_string(), 1, None, None),
        TestData("-".to_string(), -1, None, None),
        TestData("1-10,21-30".to_string(), 25, None, Some(15)),
        TestData("1-10,21-30".to_string(), -25, None, None),
        // at_pos
        TestData("0-9".to_string(), 1, Some(0), Some(2)),
        TestData("0-9".to_string(), 6, Some(5), Some(7)),
        TestData("0-9".to_string(), 10, Some(9), None),
        TestData("0-9".to_string(), 11, None, None),
        // at_neg
        TestData("0-9".to_string(), -1, Some(9), None),
        TestData("0-9".to_string(), -5, Some(5), None),
        TestData("0-9".to_string(), -10, Some(0), None),
        TestData("0-9".to_string(), -11, None, None),
        // at_pos
        TestData("1-10,21-30,41-50".to_string(), 6, Some(6), Some(6)),
        TestData("1-10,21-30,41-50".to_string(), 16, Some(26), None),
        TestData("1-10,21-30,41-50".to_string(), 26, Some(46), Some(16)),
        TestData("1-10,21-30,41-50".to_string(), 31, None, None),
        // at_neg
        TestData("1-10,21-30,41-50".to_string(), -1, Some(50), None),
        TestData("1-10,21-30,41-50".to_string(), -11, Some(30), None),
        TestData("1-10,21-30,41-50".to_string(), -21, Some(10), None),
        TestData("1-10,21-30,41-50".to_string(), -30, Some(1), None),
        TestData("1-10,21-30,41-50".to_string(), -31, None, None),
    ];

    for t in tests.iter() {
        let set = IntSpan::from(&t.0);

        // at
        if t.2.is_some() {
            assert_eq!(set.at(t.1), t.2.unwrap());
        }

        // index
        if t.3.is_some() {
            assert_eq!(set.index(t.1), t.3.unwrap());
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
