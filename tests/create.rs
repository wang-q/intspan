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
