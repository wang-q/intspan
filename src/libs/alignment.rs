use crate::*;
use anyhow::{anyhow, bail};
use bio::io::fasta;
use itertools::Itertools;
use std::cmp::min;
use std::collections::{BTreeMap, HashSet};
use std::io::{BufRead, Write};
use std::process::Command;
use std::str;
use std::string::String;

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
/// assert_eq!(intspan::alignment_stat(&seqs), (10, 10, 1, 0, 0, 0.1,));
///
/// let seqs = vec![
///     //*          * *
///     b"TTAGCCGCTGAGAAGCC".as_ref(),
///     b"GTAGCCGCTGA-AGGCC".as_ref(),
/// ];
/// assert_eq!(intspan::alignment_stat(&seqs), (17, 16, 2, 1, 0, 0.125,));
///
/// let seqs = vec![
///     //    * **    *   ** *   *
///     b"GATTATCATCACCCCAGCCACATW".as_ref(),
///     b"GATTTT--TCACTCCATTCGCATA".as_ref(),
/// ];
/// assert_eq!(intspan::alignment_stat(&seqs), (24, 21, 5, 2, 1, 0.238,));
///
/// ```
pub fn alignment_stat(seqs: &[&[u8]]) -> (i32, i32, i32, i32, i32, f32) {
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

        if column.iter().all(|e| BASES.contains(e)) {
            comparable += 1;
            if column.iter().any(|e| *e != column[0]) {
                difference += 1;
            }
        } else if column.iter().any(|e| *e == b'-') {
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

/// pos, tbase, qbase, bases, mutant_to, freq, occured
///
/// ```
/// let seqs = vec![
///     //        *
///     b"AAAATTTTGG".as_ref(),
///     b"aaaatttttg".as_ref(),
/// ];
/// assert_eq!(intspan::get_subs(&seqs).unwrap().first().unwrap(), &(
///     9,
///     "G".to_string(),
///     "T".to_string(),
///     "GT".to_string(),
///     "G<->T".to_string(),
///     1,
///     "10".to_string()
/// ));
///
/// let seqs = vec![
///     //*   **     * *
///     b"TTAG--GCTGAGAAGC".as_ref(),
///     b"GTAGCCGCTGA-AGGC".as_ref(),
/// ];
/// assert_eq!(intspan::get_subs(&seqs).unwrap().first().unwrap(), &(
///     1,
///     "T".to_string(),
///     "G".to_string(),
///     "TG".to_string(),
///     "T<->G".to_string(),
///     1,
///     "10".to_string()
/// ));
/// assert_eq!(intspan::get_subs(&seqs).unwrap().get(1).unwrap(), &(
///     14,
///     "A".to_string(),
///     "G".to_string(),
///     "AG".to_string(),
///     "A<->G".to_string(),
///     1,
///     "10".to_string()
/// ));
///
/// ```
pub fn get_subs(
    seqs: &[&[u8]],
) -> anyhow::Result<Vec<(i32, String, String, String, String, i32, String)>> {
    let seq_count = seqs.len();
    let length = seqs[0].len();

    // For each position, search for polymorphic sites
    let mut bases_of: BTreeMap<usize, Vec<u8>> = BTreeMap::new();
    for pos in 0..length {
        let mut column = vec![];
        for i in 0..seq_count {
            column.push(seqs[i][pos].to_ascii_uppercase());
        }

        if column.iter().all(|e| BASES.contains(e)) {
            // comparable += 1;
            if column.iter().any(|e| *e != column[0]) {
                // difference += 1;
                bases_of.insert(pos, column);
            }
        }
    }

    let mut sites = vec![];
    for pos in bases_of.keys().sorted() {
        let bases = bases_of.get(pos).unwrap();

        let tbase = bases.get(0).unwrap();

        let class: Vec<_> = bases.iter().unique().collect();

        if class.len() < 2 {
            bail!("No subs found in pos {}", pos);
        }

        let (freq, occured, qbase) = if class.len() > 2 {
            (-1, "unknown".to_string(), "".to_string())
        } else {
            let mut freq: i32 = 0;
            let mut occured = "".to_string();
            for base in bases {
                if tbase != base {
                    freq += 1;
                    occured += "0";
                } else {
                    occured += "1";
                }
            }
            let qbase = bases.iter().find(|e| *e != tbase).unwrap();

            (freq, occured, String::from_utf8(vec![*qbase]).unwrap())
        };

        let tbase = String::from_utf8(vec![*tbase]).unwrap();
        let mutant_to = if class.len() > 2 {
            "".to_string()
        } else {
            format!("{}<->{}", tbase, qbase).to_string()
        };

        // pos, tbase, qbase, bases, mutant_to, freq, occured
        sites.push((
            (pos + 1) as i32,
            tbase,
            qbase,
            String::from_utf8(bases.clone()).unwrap(),
            mutant_to,
            min(freq, seq_count as i32 - freq),
            occured.to_string(),
        ));
    }

    Ok(sites)
}

/// ```
/// use intspan::{indel_intspan, IntSpan, seq_intspan};
/// let tests : Vec<(&str, &str)> = vec![
///     // seq, expected
///     ("ATAA", "-"),
///     ("CcGc", "-"),
///     ("TAGggATaaC", "-"),
///     ("C-Gc", "2"),
///     ("C--c", "2-3"),
///     ("---c", "1-3"),
///     ("C---", "2-4"),
///     ("GCaN--NN--NNNaC", "5-6,9-10"),
/// ];
/// for (seq, expected) in tests {
///     let result = indel_intspan(seq.as_ref());
///     assert_eq!(result.to_string(), expected.to_string());
/// }
/// ```
pub fn indel_intspan(seq: &[u8]) -> IntSpan {
    let mut positions = vec![];

    for (i, base) in seq.iter().enumerate() {
        if *base == b'-' {
            positions.push(i as i32 + 1);
        }
    }

    let mut ints = IntSpan::new();
    ints.add_vec(&positions);

    ints
}

pub fn seq_intspan(seq: &[u8]) -> IntSpan {
    IntSpan::from_pair(1, seq.len() as i32).diff(&indel_intspan(seq))
}

/// Returns Strings to avoid lifetime issues
///
/// ```
/// match which::which("clustalw") {
///     Ok(_) => {
///         let seqs = vec![
///             //           *
///             b"TTAGCCGCTGAGAAGC".as_ref(),
///             b"TTAGCCGCTGAGAAGC".as_ref(),
///             b"TTAGCCGCTGAAAGC".as_ref(),
///         ];
///         let alns = intspan::align_seqs(&seqs, "clustalw").unwrap();
///         assert_eq!(alns[2], "TTAGCCGCTGA-AAGC".to_string());
///
///     }
///     Err(_) => {}
/// }
/// ```
// scoop install clustalw
pub fn align_seqs(seqs: &[&[u8]], aligner: &str) -> anyhow::Result<Vec<String>> {
    // find external aligner
    let mut bin = String::new();
    match aligner {
        "clustalw" => {
            for e in &["clustalw", "clustal-w", "clustalw2"] {
                if let Ok(pth) = which::which(e) {
                    bin = pth.to_string_lossy().to_string();
                    break;
                }
            }
        }
        "muscle" => {
            for e in &["muscle"] {
                if let Ok(pth) = which::which(e) {
                    bin = pth.to_string_lossy().to_string();
                    break;
                }
            }
        }
        "mafft" => {
            for e in &["mafft"] {
                if let Ok(pth) = which::which(e) {
                    bin = pth.to_string_lossy().to_string();
                    break;
                }
            }
        }
        _ => {
            return Err(anyhow!("Unrecognized aligner: {}", aligner));
        }
    };
    // eprintln!("bin = {:#?}", bin);

    if bin.is_empty() {
        return Err(anyhow!("Can't find the external command: {}", aligner));
    }

    // Create temp in/out files
    let mut seq_in = tempfile::Builder::new()
        .prefix("seq-in-")
        .suffix(".fasta")
        .rand_bytes(8)
        .tempfile()?;
    for (i, seq) in seqs.iter().enumerate() {
        write!(seq_in, ">seq-{}\n{}\n", i, str::from_utf8(seq).unwrap())?;
    }
    let seq_in_path = seq_in.into_temp_path();

    let seq_out = tempfile::Builder::new()
        .prefix("seq-out-")
        .suffix(".fasta")
        .rand_bytes(8)
        .tempfile()?;
    let seq_out_path = seq_out.into_temp_path();

    // eprintln!("seq_in_path = {:#?}", seq_in_path);

    // Run
    let output = match aligner {
        "clustalw" => Command::new(bin)
            .arg("-align")
            .arg("-type=dna")
            .arg("-output=fasta")
            .arg("-outorder=input")
            .arg("-quiet")
            .arg(format!("-infile={}", seq_in_path.to_string_lossy()))
            .arg(format!("-outfile={}", seq_out_path.to_string_lossy()))
            .output()?,
        "muscle" => Command::new(bin)
            .arg("-quiet")
            .arg("-in")
            .arg(seq_in_path.to_string_lossy().to_string())
            .arg("-out")
            .arg(seq_out_path.to_string_lossy().to_string())
            .output()?,
        "mafft" => Command::new(bin)
            .arg("-quiet")
            .arg("-auto")
            .arg(seq_in_path.to_string_lossy().to_string())
            .arg(">")
            .arg(seq_out_path.to_string_lossy().to_string())
            .output()?,
        _ => unreachable!(),
    };

    // eprintln!("output = {:#?}", output);

    if !output.status.success() {
        return Err(anyhow!("Command executed with failing error code"));
    }

    // Load outputs
    let mut out_seq = vec![];
    let reader = reader(seq_out_path.to_string_lossy().as_ref());
    let fa_in = fasta::Reader::new(reader);
    for result in fa_in.records() {
        // obtain record or fail with error
        let record = result.unwrap();
        out_seq.push(String::from_utf8(record.seq().to_vec()).unwrap());
    }

    // closing the `TempPath` explicitly
    seq_in_path.close()?;
    seq_out_path.close()?;

    Ok(out_seq)
}

/// ```
/// match which::which("spoa") {
///     Ok(_) => {
///         let seqs = vec![
///         //              *
///             b"TTAGCCGCTGAGAAGC".as_ref(),
///             b"TTAGCCGCTGAGAAGC".as_ref(),
///             b"TTAGCCGCTGA-AAGC".as_ref(),
///         ];
///         let cons = intspan::get_consensus_poa(&seqs).unwrap();
///         assert_eq!(cons, "TTAGCCGCTGAGAAGC".to_string());
///
///         let seqs = vec![
///         //      *   **
///             b"AAATTTTGG".as_ref(),
///             b"AAAATTTTT".as_ref(),
///         ];
///         let cons = intspan::get_consensus_poa(&seqs).unwrap();
///         assert_eq!(cons, "AAAATTTTGG".to_string());
///
///         let seqs = vec![
///         //           **
///             b"AAAATTTTGG".as_ref(),
///             b"AAAATTTTTG".as_ref(),
///         ];
///         let cons = intspan::get_consensus_poa(&seqs).unwrap();
///         assert_eq!(cons, "AAAATTTTTG".to_string());
///
///         let seqs = vec![
///         //
///             b"AAAATTTTGG".as_ref(),
///         ];
///         let cons = intspan::get_consensus_poa(&seqs).unwrap();
///         assert_eq!(cons, "AAAATTTTGG".to_string());
///
///     }
///     Err(_) => {}
/// }
/// ```
// cargo test --doc utils::get_consensus_poa
pub fn get_consensus_poa(seqs: &[&[u8]]) -> anyhow::Result<String> {
    let mut bin = String::new();
    for e in &["spoa"] {
        if let Ok(pth) = which::which(e) {
            bin = pth.to_string_lossy().to_string();
            break;
        }
    }

    if bin.is_empty() {
        return Err(anyhow!("Can't find the external command"));
    }

    let mut seq_in = tempfile::Builder::new()
        .prefix("seq-in-")
        .suffix(".fasta")
        .rand_bytes(8)
        .tempfile()?;

    for (i, seq) in seqs.iter().enumerate() {
        write!(seq_in, ">seq-{}\n{}\n", i, str::from_utf8(seq).unwrap())?;
    }
    let seq_in_path = seq_in.into_temp_path();

    let mut seq = String::new();
    let output = Command::new(bin)
        .arg("--result")
        .arg("0")
        .arg(seq_in_path.to_string_lossy().to_string())
        .output()?;

    if !output.status.success() {
        return Err(anyhow!("Command executed with failing error code"));
    }

    // closing the `TempPath` explicitly
    seq_in_path.close()?;

    for line in output.stdout.lines().map_while(Result::ok) {
        // header
        if line.starts_with('>') {
            continue;
        }

        seq += line.as_str();
    }

    Ok(seq)
}

/// Coordinate transforming - from chr to align
///
/// ```
/// use intspan::{indel_intspan, IntSpan, seq_intspan};
/// let tests : Vec<(&str, i32, i32, &str, i32)> = vec![
///     // seq, pos, chr_start, strand, expected
///     ("AAAATTTTTG", 4, 1, "+", 4),
///     ("AAAATTTTTG", 4, 1, "-", 7),
///     ("-AA--TTTGG", 5, 1, "+", 8),
///     ("-AA--TTTGG", 5, 1, "-", 6),
///     ("-AA--TTTGG", 105, 101, "+", 8),
///     ("-AA--TTTGG", 105, 101, "-", 6),
/// ];
/// for (seq, pos, chr_start, strand, expected) in tests {
///     let ints = seq_intspan(seq.as_ref());
///     // eprintln!("ints.to_string() = {:#?}", ints.to_string());
///     let result = intspan::chr_to_align(&ints, pos, chr_start, strand).unwrap();
///     assert_eq!(result, expected);
/// }
/// ```
pub fn chr_to_align(ints: &IntSpan, pos: i32, chr_start: i32, strand: &str) -> anyhow::Result<i32> {
    let chr_end = chr_start + ints.size() - 1;

    if pos < chr_start || pos > chr_end {
        return Err(anyhow!(
            "[{}] out of ranges [{}, {}]",
            pos,
            chr_start,
            chr_end
        ));
    }

    let aln_pos = match strand {
        "+" => ints.at(pos - chr_start + 1),
        "-" => ints.at(-(pos - chr_start + 1)),
        _ => {
            return Err(anyhow!("Unrecognized strand: {}", strand));
        }
    };

    Ok(aln_pos)
}

/// Coordinate transforming - from align to chr
///
/// ```
/// use intspan::{indel_intspan, IntSpan, seq_intspan};
/// let data : Vec<(&str, i32, i32, &str, i32)> = vec![
///     // seq, pos, chr_start, strand, expected
///     ("AAAATTTTTG", 4, 1, "+", 4),
///     ("AAAATTTTTG", 4, 1, "-", 7),
///     ("AAAATTTTTG", 4, 101, "+", 104),
///     ("AAAATTTTTG", 4, 101, "-", 107),
///     ("-AA--TTTGG", 6, 1, "+", 3),
///     ("-AA--TTTGG", 6, 1, "-", 5),
///     ("-AA--TTTGG", 6, 101, "+", 103),
///     ("-AA--TTTGG", 6, 101, "-", 105),
///     ("-AA--TTTGG", 1, 1, "+", 1),
///     ("-AA--TTTGG", 1, 1, "-", 7),
///     ("-AA--TTTGG-", 10, 1, "+", 7),
///     ("-AA--TTTGG-", 10, 1, "-", 1),
///     ("-AA--TTTGG", 4, 101, "+", 102),
///     ("-AA--TTTGG", 4, 101, "-", 106),
/// ];
/// for (seq, pos, chr_start, strand, expected) in data {
///     let ints = seq_intspan(seq.as_ref());
///     // eprintln!("ints.to_string() = {:#?}", ints.to_string());
///     let result = intspan::align_to_chr(&ints, pos, chr_start, strand).unwrap();
///     assert_eq!(result, expected);
/// }
/// ```
pub fn align_to_chr(ints: &IntSpan, pos: i32, chr_start: i32, strand: &str) -> anyhow::Result<i32> {
    let chr_end = chr_start + ints.size() - 1;

    if pos < 1 {
        return Err(anyhow!("align pos [{}] out of ranges", pos,));
    }

    let mut chr_pos = if ints.contains(pos) {
        ints.index(pos)
    } else if pos < ints.min() {
        1
    } else if pos > ints.max() {
        ints.size()
    } else {
        // pos is in the holes
        // pins to the left base
        let spans = ints.spans();
        let mut cursor = pos;
        for i in 0..spans.len() {
            if spans[i].1 < cursor {
                continue;
            } else {
                cursor = spans[i - 1].1;
                break;
            }
        }

        ints.index(cursor)
    };

    chr_pos = match strand {
        "+" => chr_pos + chr_start - 1,
        "-" => chr_end - chr_pos + 1,
        _ => {
            return Err(anyhow!("Unrecognized strand: {}", strand));
        }
    };

    Ok(chr_pos)
}

/// Trims pure dash regions
///
/// ```
/// let mut seqs = vec![
///     "AAAATTTTTG".to_string(),
///     "AAAATTTTTG".to_string(),
///     "AAAATTTTTG".to_string(),
/// ];
/// intspan::trim_pure_dash(&mut seqs);
/// assert_eq!(seqs[0].len(), 10);
///
/// let mut seqs = vec![
///     "-AA--TTTGG".to_string(),
///     "-AA--TTTGG".to_string(),
///     "-AA--TTTGG".to_string(),
/// ];
/// intspan::trim_pure_dash(&mut seqs);
/// assert_eq!(seqs[0].len(), 7);
///
/// let mut seqs = vec![
///     "-AA--TTTGG".to_string(),
///     "-AAA-TTTGG".to_string(),
///     "AAA--TTTTG".to_string(),
/// ];
/// intspan::trim_pure_dash(&mut seqs);
/// assert_eq!(seqs[0].len(), 9);
///
/// ```
pub fn trim_pure_dash(seqs: &mut Vec<String>) {
    let mut trim_region = IntSpan::new();
    let seq_count = seqs.len();

    for seq in seqs.iter() {
        let ints = indel_intspan(seq.as_bytes().to_vec().as_ref());
        if trim_region.is_empty() {
            trim_region.merge(&ints);
        } else {
            trim_region = trim_region.intersect(&ints);
        }
    }

    // trim all segments in trim_region
    for (lower, upper) in trim_region.spans().iter().rev() {
        for i in 0..seq_count {
            seqs[i].replace_range((*lower as usize - 1)..*upper as usize, "");
        }
    }
}

/// Trims outgroup-only regions
///
/// Iff. intersect is superset of union
///     T G----C
///     Q G----C
///     O GAAAAC
///
/// ```
/// let mut seqs = vec![
///     "AAAATTTTTG".to_string(),
///     "AAAATTTTTG".to_string(),
///     "AAAATTTTTG".to_string(),
/// ];
/// intspan::trim_outgroup(&mut seqs);
/// assert_eq!(seqs[0].len(), 10);
///
/// let mut seqs = vec![
///     "-AA--TTTGG".to_string(),
///     "-AA--TTTGG".to_string(),
///     "-AA--TTTGG".to_string(),
/// ];
/// intspan::trim_outgroup(&mut seqs);
/// assert_eq!(seqs[0].len(), 7);
///
/// let mut seqs = vec![
///     "-AA--TTTGG".to_string(),
///     "-AAA-TTTGG".to_string(),
///     "AAA--TTTTG".to_string(),
/// ];
/// intspan::trim_outgroup(&mut seqs);
/// assert_eq!(seqs[0].len(), 9);
///
/// let mut seqs = vec![
///     "AAA--TT-GG".to_string(),
///     "AAAATTT-GG".to_string(),
///     "AAA--TTTTG".to_string(),
/// ];
/// intspan::trim_outgroup(&mut seqs);
/// assert_eq!(seqs[0].len(), 9);
///
/// let mut seqs = vec![
///     "-AA--TT-GG".to_string(),
///     "-AAA-TT-GG".to_string(),
///     "AAA--TTTTG".to_string(),
/// ];
/// intspan::trim_outgroup(&mut seqs);
/// assert_eq!(seqs[0].len(), 8);
///
/// ```
pub fn trim_outgroup(seqs: &mut Vec<String>) {
    let seq_count = seqs.len();

    assert!(seq_count >= 3, "Need three or more sequences");

    // Last seq is the outgroup
    let (union_ints, intersect_ints) = align_indel_ints(seqs, seq_count - 1);

    // find trim_region
    let mut trim_region = IntSpan::new();
    for (lower, upper) in union_ints.spans() {
        let ints = IntSpan::from_pair(lower, upper);
        if intersect_ints.superset(&ints) {
            trim_region.merge(&ints);
        }
    }

    // trim all segments in trim_region
    for (lower, upper) in trim_region.spans().iter().rev() {
        for i in 0..seq_count {
            seqs[i].replace_range((*lower as usize - 1)..*upper as usize, "");
        }
    }
}

/// Records complex ingroup indels (ingroup-outgroup complex indels are not identified here)
///
/// After trim_outgroup(), All ingroup intersect ints are parts of complex indels
/// intersect 4-5, union 2-5
///     T GGA--C
///     Q G----C
///     O GGAGAC
/// result, complex_region 2-3
///     T GGAC
///     Q G--C
///     O GGAC
///
/// ```
/// let mut seqs = vec![
///     "AAAATTTTTG".to_string(),
///     "AAAATTTTTG".to_string(),
///     "AAAATTTTTG".to_string(),
/// ];
/// intspan::trim_outgroup(&mut seqs);
/// let complex = intspan::trim_complex_indel(&mut seqs);
/// assert_eq!(seqs[0].len(), 10);
/// assert_eq!(complex.to_string(), "-");
///
/// let mut seqs = vec![
///     "-AA--TTTGG".to_string(),
///     "-AA--TTTGG".to_string(),
///     "-AA--TTTGG".to_string(),
/// ];
/// intspan::trim_outgroup(&mut seqs);
/// let complex = intspan::trim_complex_indel(&mut seqs);
/// assert_eq!(seqs[0].len(), 7);
/// assert_eq!(complex.to_string(), "-");
///
/// let mut seqs = vec![
///     "-AA--TTTGG".to_string(),
///     "-AAA-TTTGG".to_string(),
///     "AAA--TTTGG".to_string(),
/// ];
/// intspan::trim_outgroup(&mut seqs);
/// let complex = intspan::trim_complex_indel(&mut seqs);
/// assert_eq!(seqs[0].len(), 8);
/// assert_eq!(complex.to_string(), "3");
///
/// let mut seqs = vec![
///     "AAA--TT-GG".to_string(),
///     "AAAATTT-GG".to_string(),
///     "AAA--TTTTG".to_string(),
/// ];
/// intspan::trim_outgroup(&mut seqs);
/// let complex = intspan::trim_complex_indel(&mut seqs);
/// assert_eq!(seqs[0].len(), 9);
/// assert_eq!(complex.to_string(), "-");
///
/// let mut seqs = vec![
///     "-AA--TT-GG".to_string(),
///     "-AAA-TT-GG".to_string(),
///     "AAA--TTTTG".to_string(),
/// ];
/// intspan::trim_outgroup(&mut seqs);
/// let complex = intspan::trim_complex_indel(&mut seqs);
/// assert_eq!(seqs[0].len(), 7);
/// assert_eq!(complex.to_string(), "3");
///
/// let mut seqs = vec![
///     "-AA--TTTGG".to_string(),
///     "-AAA-TT-GG".to_string(),
///     "AAA--TTTTG".to_string(),
/// ];
/// intspan::trim_outgroup(&mut seqs);
/// let complex = intspan::trim_complex_indel(&mut seqs);
/// assert_eq!(seqs[0].len(), 8);
/// assert_eq!(complex.to_string(), "3");
///
/// let mut seqs = vec![
///     "-AA--TTTGG".to_string(),
///     "-AAA-TT-GG".to_string(),
///     "AAA--TT--G".to_string(),
/// ];
/// intspan::trim_outgroup(&mut seqs);
/// let complex = intspan::trim_complex_indel(&mut seqs);
/// assert_eq!(seqs[0].len(), 8);
/// assert_eq!(complex.to_string(), "3");
///
/// let mut seqs = vec![
///     "-AA--TTTGG".to_string(),
///     "-AAA-TT--G".to_string(),
///     "AAA--TT-GG".to_string(),
/// ];
/// intspan::trim_outgroup(&mut seqs);
/// let complex = intspan::trim_complex_indel(&mut seqs);
/// assert_eq!(seqs[0].len(), 8);
/// assert_eq!(complex.to_string(), "3");
///
/// ```
pub fn trim_complex_indel(seqs: &mut Vec<String>) -> String {
    let seq_count = seqs.len();

    assert!(seq_count >= 3, "Need three or more sequences");

    // Last seq is the outgroup
    let (union_ints, intersect_ints) = align_indel_ints(seqs, seq_count - 1);

    // find ingroup complex_region
    let mut complex_region = IntSpan::new();
    for (lower, upper) in intersect_ints.spans().iter().rev() {
        let sub_intersect_ints = IntSpan::from_pair(*lower, *upper);

        // trim sequences, including outgroup
        for i in 0..seq_count {
            seqs[i].replace_range((*lower as usize - 1)..*upper as usize, "");
        }

        // add to complex_region
        for sub_union_ints in union_ints.intses() {
            if sub_union_ints.superset(&sub_intersect_ints) {
                complex_region.merge(&sub_union_ints);
            }
        }

        // modify all related set
        // intersect_ints is not affected
        // union_ints is affected
        complex_region = complex_region.banish(*lower, *upper);
    }

    return complex_region.to_string();
}

fn align_indel_ints(seqs: &mut Vec<String>, count: usize) -> (IntSpan, IntSpan) {
    let mut union_ints = IntSpan::new();
    let mut intersect_ints = IntSpan::new();

    for i in 0..count {
        let ints = indel_intspan(seqs[i].as_bytes().to_vec().as_ref());

        if union_ints.is_empty() {
            union_ints.merge(&ints);
        } else {
            union_ints = union_ints.union(&ints);
        }

        if intersect_ints.is_empty() {
            intersect_ints.merge(&ints);
        } else {
            intersect_ints = intersect_ints.intersect(&ints);
        }
    }

    (union_ints, intersect_ints)
}
