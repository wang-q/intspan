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
