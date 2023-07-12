use crate::Range;
use std::collections::VecDeque;
use std::io;

pub struct LinesRef<'a, B: 'a> {
    buf: &'a mut B,
}

impl<'a, B: io::BufRead> Iterator for LinesRef<'a, B> {
    type Item = io::Result<String>;

    fn next(&mut self) -> Option<io::Result<String>> {
        let mut buf = String::new();
        match self.buf.read_line(&mut buf) {
            Ok(0) => None,
            Ok(_n) => {
                if buf.ends_with('\n') {
                    buf.pop();
                    if buf.ends_with('\r') {
                        buf.pop();
                    }
                }
                Some(Ok(buf))
            }
            Err(e) => Some(Err(e)),
        }
    }
}

/// Indicates one of the two strands.
fn parse_strand(strand: &str) -> Result<String, io::Error> {
    match strand {
        "+" => Ok("+".to_string()),
        "-" => Ok("-".to_string()),
        _ => Err(io::Error::new(io::ErrorKind::Other, "Strand not valid")),
    }
}

#[derive(Default, Clone)]
pub struct FasEntry {
    pub range: Range,
    pub seq: Vec<u8>,
}

impl FasEntry {
    // Immutable accessors
    pub fn range(&self) -> &Range {
        &self.range
    }
    pub fn seq(&self) -> &Vec<u8> {
        &self.seq
    }

    pub fn new() -> Self {
        Self {
            range: Range::new(),
            seq: vec![],
        }
    }

    /// Constructed from range and seq
    ///
    /// ```
    /// # use intspan::Range;
    /// # use intspan::FasEntry;
    /// let range = Range::from("I", 1, 10);
    /// let seq = "ACAGCTGA-AA".as_bytes().to_vec();
    /// let entry = FasEntry::from(&range, &seq);
    /// # assert_eq!(*entry.range.chr(), "I");
    /// # assert_eq!(*entry.range.start(), 1);
    /// # assert_eq!(*entry.range.end(), 10);
    /// # assert_eq!(String::from_utf8(entry.seq).unwrap(), "ACAGCTGA-AA".to_string());
    /// ```
    pub fn from(range: &Range, seq: &[u8]) -> Self {
        Self {
            range: range.clone(),
            seq: seq.to_owned(),
        }
    }
}

/// A Fas alignment block.
pub struct FasBlock {
    pub entries: Vec<FasEntry>,
    pub names: Vec<String>,
}

/// Get the next FasBlock out of the input.
pub fn next_fas_block<T: io::BufRead + ?Sized>(mut input: &mut T) -> Result<FasBlock, io::Error> {
    let mut header: Option<String> = None;
    {
        let lines = LinesRef { buf: &mut input };
        for line_res in lines {
            let line: String = line_res?;
            if line.trim().is_empty() {
                // Blank line
                continue;
            }
            if line.starts_with('#') {
                // Fas comment
                continue;
            } else if line.starts_with('>') {
                // Start of a block
                header = Some(line);
                break;
            } else {
                // Shouldn't see this.
                return Err(io::Error::new(io::ErrorKind::Other, "Unexpected line"));
            }
        }
    }
    let block = parse_fas_block(
        header.ok_or(io::Error::new(io::ErrorKind::Other, "EOF"))?,
        LinesRef { buf: &mut input },
    )?;
    Ok(block)
}

pub fn parse_fas_block(
    header: String,
    iter: impl Iterator<Item = Result<String, io::Error>>,
) -> Result<FasBlock, io::Error> {
    let mut block_lines: VecDeque<String> = VecDeque::new();
    block_lines.push_back(header);

    for line_res in iter {
        let line: String = line_res?;
        if line.is_empty() {
            // Blank lines terminate the "paragraph".
            break;
        }
        block_lines.push_back(line);
    }
    let mut block_entries: Vec<FasEntry> = vec![];
    let mut block_names: Vec<String> = vec![];

    while let Some(header) = block_lines.pop_front() {
        let range = Range::from_str(header.as_str());
        let seq = block_lines.pop_front().unwrap().as_bytes().to_vec();

        let entry = FasEntry::from(&range, &seq);
        block_entries.push(entry);
        block_names.push(range.name().to_string());
    }

    Ok(FasBlock {
        entries: block_entries,
        names: block_names,
    })
}

#[cfg(test)]
mod fas_tests {
    use super::*;
    use std::io::{BufRead, BufReader};

    #[test]
    fn parse_block_range() {
        let str = ">S288c.I(+):13267-13287|species=S288c
TCGTCAGTTGGTTGACCATTA
>YJM789.gi_151941327(-):5668-5688|species=YJM789
TCGTCAGTTGGTTGACCATTA
>RM11.gi_61385832(-):5590-5610|species=RM11
TCGTCAGTTGGTTGACCATTA
>Spar.gi_29362400(+):2477-2497|species=Spar
TCATCAGTTGGCAAACCGTTA

>S288c.I(+):185273-185334|species=S288c
GCATATAATATGAACCAATATCTA-TTCATGAAGAGACTATGGTATACCCGGTACTATTTCTA
>YJM789.gi_151941327(+):156665-156726|species=YJM789
GCGTATAATATGAACCAGTATCTTTTTCATGAAG-GGCTATGGTATACTCCATATTACTTCTA
>RM11.gi_61385833(-):3668-3730|species=RM11
GCATATAATATGAACCAATATCTATTTCATGGAGAGACTATGATAT-CCCCGTACTATTTCTA
>Spar.gi_29362478(-):2102-2161|species=Spar
GC-TAAAATATGAA-CGATATTTA-CCTGTAGAGGGACTATGGGAT-CCCCATACTACTTT--
";
        let mut reader = BufReader::new(str.as_bytes());
        let block = next_fas_block(&mut reader).unwrap();
        assert_eq!(
            block.entries.get(0).unwrap().range.to_string(),
            "S288c.I(+):13267-13287".to_string()
        );
        assert_eq!(
            block.entries.get(2).unwrap().range.to_string(),
            "RM11.gi_61385832(-):5590-5610".to_string()
        );

        let block = next_fas_block(&mut reader).unwrap();
        assert_eq!(
            String::from_utf8(block.entries.get(1).unwrap().seq.clone()).unwrap(),
            "GCGTATAATATGAACCAGTATCTTTTTCATGAAG-GGCTATGGTATACTCCATATTACTTCTA".to_string()
        );
    }
}
// MAF
// https://genome.ucsc.edu/FAQ/FAQformat.html#format5
// https://github.com/joelarmstrong/maf_stream/blob/master/multiple_alignment_format/src/parser.rs

/// An alignment entry within a MAF block. Corresponds to the "s" line.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MafEntry {
    /// Actual sequence of bases/amino acids, including gaps.
    pub alignment: Vec<u8>,
    /// The sequence name.
    pub src: String,
    /// Start of the aligned region within this sequence.
    pub start: u64,
    /// Length of the aligned region (not including gaps).
    pub size: u64,
    /// The total length of this sequence (including regions outside
    /// this alignment).
    pub src_size: u64,
    /// Which strand the aligned sequence is on.
    pub strand: String,
}

impl MafEntry {
    /// create a range string from a MAF entry
    pub fn to_range(&self) -> String {
        let mut range = String::new();

        // adjust coordinates to be one-based inclusive
        let mut start = self.start + 1;
        let mut end = start + self.size - 1;

        // If the strand field is "-" then this is the start relative to the reverse-complemented source sequence
        if self.strand == *"-" {
            start = self.src_size - start + 1;
            end = self.src_size - end + 1;
            (start, end) = (end, start);
        }

        range += self.src.as_str();

        range += "(";
        range += self.strand.as_str();
        range += ")";
        range += ":";

        range += start.to_string().as_str();
        range += "-";
        range += end.to_string().as_str();

        range
    }
}

/// A MAF alignment block.
#[derive(Debug, PartialEq, Eq)]
pub struct MafBlock {
    pub entries: Vec<MafEntry>,
}

/// Get the next MafBlock out of the input.
pub fn next_maf_block<T: io::BufRead + ?Sized>(mut input: &mut T) -> Result<MafBlock, io::Error> {
    let mut header: Option<String> = None;
    {
        let lines = LinesRef { buf: &mut input };
        for line_res in lines {
            let line: String = line_res?;
            if line.trim().is_empty() {
                // Blank line
                continue;
            }
            if line.starts_with('#') {
                // MAF comment
                // return Ok(MAFBlock { entries: vec![] });
                continue;
            } else if line.starts_with('a') {
                // Start of a block
                header = Some(line);
                break;
            } else {
                // Shouldn't see this.
                return Err(io::Error::new(io::ErrorKind::Other, "Unexpected line"));
            }
        }
    }
    let block = parse_maf_block(
        header.ok_or(io::Error::new(io::ErrorKind::Other, "EOF"))?,
        LinesRef { buf: &mut input },
    )?;
    Ok(block)
}

fn parse_s_line(
    fields: &mut Vec<&str>,
    block_entries: &mut Vec<MafEntry>,
) -> Result<(), io::Error> {
    let alignment = fields
        .pop()
        .ok_or(io::Error::new(io::ErrorKind::Other, "s line incomplete"))?;
    let src_size = fields
        .pop()
        .ok_or(io::Error::new(io::ErrorKind::Other, "s line incomplete"))
        .and_then(|s| {
            s.parse::<u64>()
                .map_err(|_| io::Error::new(io::ErrorKind::Other, "invalid sequence size"))
        })?;
    let strand = fields
        .pop()
        .ok_or(io::Error::new(io::ErrorKind::Other, "s line incomplete"))
        .and_then(parse_strand)?;
    let aligned_length = fields
        .pop()
        .ok_or(io::Error::new(io::ErrorKind::Other, "s line incomplete"))
        .and_then(|s| {
            s.parse::<u64>()
                .map_err(|_| io::Error::new(io::ErrorKind::Other, "invalid aligned length"))
        })?;
    let start = fields
        .pop()
        .ok_or(io::Error::new(io::ErrorKind::Other, "s line incomplete"))
        .and_then(|s| {
            s.parse::<u64>()
                .map_err(|_| io::Error::new(io::ErrorKind::Other, "invalid start"))
        })?;
    let src = fields
        .pop()
        .ok_or(io::Error::new(io::ErrorKind::Other, "s line incomplete"))?;
    block_entries.push(MafEntry {
        alignment: alignment.as_bytes().to_vec(),
        src: src.to_string(),
        start,
        size: aligned_length,
        src_size,
        strand,
    });
    Ok(())
}

pub fn parse_maf_block(
    header: String,
    iter: impl Iterator<Item = Result<String, io::Error>>,
) -> Result<MafBlock, io::Error> {
    let mut block_lines = vec![];
    block_lines.push(header);

    for line_res in iter {
        let line: String = line_res?;
        if line.is_empty() {
            // Blank lines terminate the "paragraph".
            break;
        }
        block_lines.push(line);
    }
    let mut block_entries: Vec<MafEntry> = vec![];

    for line in block_lines {
        let mut fields: Vec<_> = line.split_whitespace().collect();
        match fields[0] {
            "a" => (),
            "s" => parse_s_line(&mut fields, &mut block_entries)?,
            "i" => (),
            "e" => (),
            "q" => (),
            "track" => (),
            _ => return Err(io::Error::new(io::ErrorKind::Other, "BadLineType")),
        };
    }

    Ok(MafBlock {
        entries: block_entries,
    })
}

#[cfg(test)]
mod maf_tests {
    use super::*;
    use std::io::{BufRead, BufReader};

    #[test]
    fn parse_comment() {
        let str = "##maf version=1";
        let mut reader = BufReader::new(str.as_bytes());
        let res = next_maf_block(&mut reader);
        eprintln!("got error {:?}", res.as_ref().err());
        assert!(matches!(res.unwrap_err().kind(), io::ErrorKind::Other));
    }

    #[test]
    fn parse_blank_comment() {
        let str = "#";
        let mut reader = BufReader::new(str.as_bytes());
        let res = next_maf_block(&mut reader);
        assert!(matches!(res.unwrap_err().kind(), io::ErrorKind::Other));
    }

    #[test]
    fn parse_err_unexpected() {
        let str = "#\nUnexpected";
        let mut reader = BufReader::new(str.as_bytes());
        let res = next_maf_block(&mut reader);
        eprintln!("got error {:?}", res.as_ref().err());
        assert!(matches!(res.unwrap_err().kind(), io::ErrorKind::Other));
    }

    #[test]
    fn parse_err_s() {
        let str = "#\na\ns 123";
        let mut reader = BufReader::new(str.as_bytes());
        let res = next_maf_block(&mut reader);
        eprintln!("got error {:?}", res.as_ref().err());
        assert!(matches!(res.unwrap_err().kind(), io::ErrorKind::Other));
    }

    #[test]
    fn parse_block_a() {
        let str = "#\na score=23262.0 pass=2";
        let mut reader = BufReader::new(str.as_bytes());
        match next_maf_block(&mut reader) {
            Err(e) => assert!(false, "Got error {:?}", e),
            Ok(val) => assert_eq!(val, MafBlock { entries: vec![] }),
        }
    }

    #[test]
    fn parse_block_a_empty() {
        let str = "#\na";
        let mut reader = BufReader::new(str.as_bytes());
        match next_maf_block(&mut reader) {
            Err(e) => assert!(false, "Got error {:?}", e),
            Ok(val) => assert_eq!(val, MafBlock { entries: vec![] }),
        }
    }

    #[test]
    fn parse_block_s_lines() {
        let str = "a meta1=val1 meta2=val2
s hg16.chr7    27707221 13 + 158545518 gcagctgaaaaca
s baboon         249182 12 -   4622798 gcagctgaa-aca
i baboon       I 234 n 19
s mm4.chr6     53310102 12 + 151104725 ACAGCTGA-AATA

this line is a canary to ensure it stops after a 'paragraph'";
        let mut lines = BufReader::new(str.as_bytes()).lines();
        let header = lines.next().unwrap().unwrap();
        match parse_maf_block(header, lines) {
            Err(e) => assert!(false, "got error {:?}", e),
            Ok(val) => assert_eq!(
                val,
                MafBlock {
                    entries: vec![
                        MafEntry {
                            src: "hg16.chr7".to_owned(),
                            start: 27707221,
                            size: 13,
                            src_size: 158545518,
                            strand: "+".to_string(),
                            alignment: "gcagctgaaaaca".as_bytes().to_vec(),
                        },
                        MafEntry {
                            src: "baboon".to_owned(),
                            start: 249182,
                            size: 12,
                            src_size: 4622798,
                            strand: "-".to_string(),
                            alignment: "gcagctgaa-aca".as_bytes().to_vec(),
                        },
                        MafEntry {
                            src: "mm4.chr6".to_owned(),
                            start: 53310102,
                            size: 12,
                            src_size: 151104725,
                            strand: "+".to_string(),
                            alignment: "ACAGCTGA-AATA".as_bytes().to_vec(),
                        },
                    ],
                }
            ),
        }
    }

    #[test]
    fn parse_block_s_range() {
        let str = "##maf version=1 scoring=multiz
a score=514600.0
s S288c.VIII          13376 34 + 562643 TTACTCGTCTTGCGGCCAAAACTCGAAGAAAAAC
s RM11_1a.scaffold_12  3529 34 + 536628 TTACTCGTCTTGCGGCCAAAACTCGAAGAAAAAC
s EC1118.FN393072_1    8746 34 + 161280 TTACTCGTCTTGCGGCCAAAACTCGAAGAAAAAC
s Spar.gi_29362578      637 33 -  73522 TTACCCGTCTTGCGTCCAAAACTCGAA-AAAAAC

a score=36468.0
s S288c.VIII          193447  99 + 562643 CG--GCATAATTTTTTCCAGGCACTTTCCGCTGCAG---TTGTTGTGCTGACAATAGTCCCATCTAGGTCAAAAAGACAAAGATCTACTGAAAATTGTGGCAtt
s RM11_1a.scaffold_12 189216 101 + 536628 CGTAACACAACTTGGTCCATGC---TTTCTCTGCGGCCACTGTTGTACTCACTATGGTACCATCTAGGTCAAAAAGACATAGATCAGCTGAAAATTCTGCCATT
s EC1118.FN393073_1    25682  99 +  44323 CG--GCATAATTTTTTCCAGGCACTTTCCGCTGCAG---TTGTTGTGCTGACAATAGTCCCATCTAGGTCAAAAAGACAAAGATCTACTGAAAATTGTGGCAtt
s Spar.gi_29362604    100946  97 - 143114 CG--ACATAGTTTTTTCCAGGCACTTTCAGCTGCGG---TTGTTGTGCTAACAATGGTCCCATCTAGGTCAAAAAGGCAGAGATCTACTGAAAATTGTGGCA--
";
        let mut reader = BufReader::new(str.as_bytes());
        let block = next_maf_block(&mut reader).unwrap();
        assert_eq!(
            block.entries.get(0).unwrap().to_range(),
            "S288c.VIII(+):13377-13410".to_string()
        );
        assert_eq!(
            block.entries.get(3).unwrap().to_range(),
            "Spar.gi_29362578(-):72853-72885".to_string()
        );

        let block = next_maf_block(&mut reader).unwrap();
        assert_eq!(
            block.entries.get(1).unwrap().to_range(),
            "RM11_1a.scaffold_12(+):189217-189317".to_string()
        );
        assert_eq!(
            block.entries.get(3).unwrap().to_range(),
            "Spar.gi_29362604(-):42072-42168".to_string()
        );
    }
}
