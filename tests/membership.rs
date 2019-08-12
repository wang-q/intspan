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
