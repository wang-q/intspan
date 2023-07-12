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
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Strand {
    Plus,
    Minus,
}

fn parse_strand(strand: &str) -> Result<Strand, io::Error> {
    match strand {
        "+" => Ok(Strand::Plus),
        "-" => Ok(Strand::Minus),
        _ => Err(io::Error::new(io::ErrorKind::Other, "Strand not valid")),
    }
}

// https://genome.ucsc.edu/FAQ/FAQformat.html#format5
// https://github.com/joelarmstrong/maf_stream/blob/master/multiple_alignment_format/src/parser.rs

/// An alignment entry within a MAF block. Corresponds to the "s" line.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MAFBlockEntry {
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
    pub strand: Strand,
}

/// A MAF alignment block.
#[derive(Debug, PartialEq, Eq)]
pub struct MAFBlock {
    pub entries: Vec<MAFBlockEntry>,
}

/// Get the next MAFItem out of the input.
pub fn next_maf_block<T: io::BufRead + ?Sized>(mut input: &mut T) -> Result<MAFBlock, io::Error> {
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
    block_entries: &mut Vec<MAFBlockEntry>,
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
    block_entries.push(MAFBlockEntry {
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
) -> Result<MAFBlock, io::Error> {
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
    let mut block_entries: Vec<MAFBlockEntry> = vec![];

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

    Ok(MAFBlock {
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
    fn parse_err_misc_s() {
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
            Ok(val) => assert_eq!(val, MAFBlock { entries: vec![] }),
        }
    }

    #[test]
    fn parse_block_a_empty() {
        let str = "#\na";
        let mut reader = BufReader::new(str.as_bytes());
        match next_maf_block(&mut reader) {
            Err(e) => assert!(false, "Got error {:?}", e),
            Ok(val) => assert_eq!(val, MAFBlock { entries: vec![] }),
        }
    }

    #[test]
    fn parse_block_s_lines() {
        let block_str = "a meta1=val1 meta2=val2
s hg16.chr7    27707221 13 + 158545518 gcagctgaaaaca
s baboon         249182 12 -   4622798 gcagctgaa-aca
i baboon       I 234 n 19
s mm4.chr6     53310102 12 + 151104725 ACAGCTGA-AATA

this line is a canary to ensure it stops after a 'paragraph'";
        let mut lines = BufReader::new(block_str.as_bytes()).lines();
        let header = lines.next().unwrap().unwrap();
        match parse_maf_block(header, lines) {
            Err(e) => assert!(false, "got error {:?}", e),
            Ok(val) => assert_eq!(
                val,
                MAFBlock {
                    entries: vec![
                        MAFBlockEntry {
                            src: "hg16.chr7".to_owned(),
                            start: 27707221,
                            size: 13,
                            src_size: 158545518,
                            strand: Strand::Plus,
                            alignment: "gcagctgaaaaca".as_bytes().to_vec(),
                        },
                        MAFBlockEntry {
                            src: "baboon".to_owned(),
                            start: 249182,
                            size: 12,
                            src_size: 4622798,
                            strand: Strand::Minus,
                            alignment: "gcagctgaa-aca".as_bytes().to_vec(),
                        },
                        MAFBlockEntry {
                            src: "mm4.chr6".to_owned(),
                            start: 53310102,
                            size: 12,
                            src_size: 151104725,
                            strand: Strand::Plus,
                            alignment: "ACAGCTGA-AATA".as_bytes().to_vec(),
                        },
                    ],
                }
            ),
        }
    }
}
