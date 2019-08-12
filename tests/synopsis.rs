use intspan::IntSpan;

#[test]
fn snippet_1() {
    let mut set = IntSpan::new();
    for i in vec![1, 2, 3, 5, 7, 9] {
        set.add_n(i);
    }
    set.add_pair(100, 10000);
    set.remove_n(1000);

    let expected = "1-3,5,7,9,100-999,1001-10000".to_string();

    assert_eq!(set.to_string(), expected);
    assert_eq!(set.cardinality(), 9906);

    assert_eq!(set.is_empty(), false);
    assert_eq!(set.is_universal(), false);
    assert_eq!(set.is_infinite(), false);
    assert_eq!(set.is_finite(), true);
    assert_eq!(set.is_pos_inf(), false);
    assert_eq!(set.is_neg_inf(), false);
}

#[test]
fn snippet_2() {
    let mut set = IntSpan::new();
    set.invert();

    let expected = format!("{}-{}", set.get_neg_inf(), set.get_pos_inf());

    assert_eq!(set.to_string(), expected);

    assert_eq!(set.is_empty(), false);
    assert_eq!(set.is_universal(), true);
    assert_eq!(set.is_infinite(), true);
    assert_eq!(set.is_finite(), false);
    assert_eq!(set.is_pos_inf(), true);
    assert_eq!(set.is_neg_inf(), true);
}

#[test]
fn snippet_3() {
    let mut set = IntSpan::new();
    set.add_pair(1, set.get_pos_inf());

    let expected = format!("{}-{}", 1, set.get_pos_inf());

    assert_eq!(set.to_string(), expected);

    assert_eq!(set.is_empty(), false);
    assert_eq!(set.is_universal(), false);
    assert_eq!(set.is_infinite(), true);
    assert_eq!(set.is_finite(), false);
    assert_eq!(set.is_pos_inf(), true);
    assert_eq!(set.is_neg_inf(), false);
}
