use itertools::Itertools;
use std::collections::HashSet;

lazy_static! {
    static ref BASES: HashSet<u8> = vec![b'a', b'g', b'c', b't', b'A', b'G', b'C', b'T',]
        .into_iter()
        .collect();
}

/// Divergence (D) between two sequences
///
/// ```
/// //           * **  **
/// let seq1 = b"GTCTGCATGCN";
/// let seq2 = b"TTTAGCTAgc-";
/// // difference 5
/// // comparable 10
/// assert_eq!(intspan::pair_d(seq1, seq2), 0.5);
/// ```
pub fn pair_d(seq1: &[u8], seq2: &[u8]) -> f32 {
    assert_eq!(
        seq1.len(),
        seq2.len(),
        "Two sequences of different length ({}!={})",
        seq1.len(),
        seq2.len()
    );

    let mut comparable = 0;
    let mut difference = 0;

    for (base1, base2) in seq1.iter().zip(seq2) {
        if BASES.contains(base1) && BASES.contains(base2) {
            comparable += 1;
            if base1.to_ascii_uppercase() != base2.to_ascii_uppercase() {
                difference += 1;
            }
        }
    }

    assert_ne!(comparable, 0, "Comparable bases shouldn't be zero");

    // eprintln!("{} {}", difference, comparable);

    difference as f32 / comparable as f32
}

/// Basic stats on alignments
///
/// ```
/// let seqs = vec![
///     //        *
///     b"AAAATTTTGG".as_ref(),
///     b"aaaatttttg".as_ref(),
/// ];
/// assert_eq!(intspan::align_stat(&seqs), (10, 10, 1, 0, 0, 0.1,));
///
/// let seqs = vec![
///     //*          * *
///     b"TTAGCCGCTGAGAAGCC".as_ref(),
///     b"GTAGCCGCTGA-AGGCC".as_ref(),
/// ];
/// assert_eq!(intspan::align_stat(&seqs), (17, 16, 2, 1, 0, 0.125,));
///
/// let seqs = vec![
///     //    * **    *   ** *   *
///     b"GATTATCATCACCCCAGCCACATW".as_ref(),
///     b"GATTTT--TCACTCCATTCGCATA".as_ref(),
/// ];
/// assert_eq!(intspan::align_stat(&seqs), (24, 21, 5, 2, 1, 0.238,));
///
/// ```
pub fn align_stat(seqs: &[&[u8]]) -> (i32, i32, i32, i32, i32, f32) {
    let seq_count = seqs.len();
    assert_ne!(seq_count, 0, "Need sequences");

    let length = seqs[0].len();

    let mut comparable = 0;
    let mut difference = 0;
    let mut gap = 0;
    let mut ambiguous = 0;

    // For each position, search for polymorphic sites
    for pos in 0..length {
        let mut column = vec![];
        for i in 0..seq_count {
            column.push(seqs[i][pos].to_ascii_uppercase());
        }
        column = column.into_iter().unique().collect();

        if column.clone().into_iter().all(|e| BASES.contains(&e)) {
            comparable += 1;
            if column.clone().into_iter().any(|e| e != column[0]) {
                difference += 1;
            }
        } else if column.clone().into_iter().any(|e| e == b'-') {
            gap += 1;
        } else {
            ambiguous += 1;
        }
    }

    assert_ne!(comparable, 0, "Comparable bases shouldn't be zero");

    let mut dists = vec![];
    for i in 0..seq_count {
        for j in i + 1..seq_count {
            let dist = pair_d(seqs[i], seqs[j]);
            dists.push(dist);
        }
    }

    let mean_d = f32::trunc(dists.iter().sum::<f32>() / dists.len() as f32 * 1000.0) / 1000.0;

    (
        length as i32,
        comparable,
        difference,
        gap,
        ambiguous,
        mean_d,
    )
}
