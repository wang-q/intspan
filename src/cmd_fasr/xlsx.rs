use clap::*;
use intspan::*;
use rust_xlsxwriter::{Color, Format, FormatAlign, FormatBorder, Workbook, XlsxError};
use std::cmp::max;
use std::collections::BTreeMap;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("xlsx")
        .about("List variations (substitutions/indels)")
        .after_help(
            r###"
* <infiles> are paths to block fasta files, .fas.gz is supported
    * infile == stdin means reading from STDIN

"###,
        )
        .arg(
            Arg::new("infiles")
                .required(true)
                .num_args(1..)
                .index(1)
                .help("Set the input files to use"),
        )
        .arg(
            Arg::new("indel")
                .long("indel")
                .action(ArgAction::SetTrue)
                .help("List indels"),
        )
        .arg(
            Arg::new("has_outgroup")
                .long("outgroup")
                .action(ArgAction::SetTrue)
                .help("There are outgroups at the end of each block"),
        )
        .arg(
            Arg::new("outfile")
                .long("outfile")
                .short('o')
                .num_args(1)
                .default_value("variations.xlsx")
                .help("Output filename"),
        )
}

// command implementation
pub fn execute(args: &ArgMatches) -> anyhow::Result<()> {
    //----------------------------
    // Args
    //----------------------------
    let outfile = args.get_one::<String>("outfile").unwrap();
    let has_outgroup = args.get_flag("has_outgroup");

    //----------------------------
    // Operating
    //----------------------------

    // Create workbook and worksheet objects
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    let format_of: BTreeMap<String, Format> = create_formats();
    let mut max_name_len = 1;
    let mut section_cursor = 1;
    let color_loop = 15;
    let wrap = 50;

    // eprintln!("format_of = {:#?}", format_of.keys());

    for infile in args.get_many::<String>("infiles").unwrap() {
        let mut reader = reader(infile);

        while let Ok(block) = next_fas_block(&mut reader) {
            let mut seqs: Vec<&[u8]> = vec![];
            for entry in &block.entries {
                seqs.push(entry.seq().as_ref());
            }

            // pos, tbase, qbase, bases, mutant_to, freq, pattern, obase
            //   0,     1,     2,     3,         4,    5,       6,     7
            let seq_count = seqs.len();
            let subs = if has_outgroup {
                let mut unpolarized = get_subs(&seqs[..(seq_count - 1)]).unwrap();
                polarize_subs(&mut unpolarized, seqs[seq_count - 1]);
                unpolarized
            } else {
                get_subs(&seqs).unwrap()
            };

            let sec_height = seq_count + 2;
            let mut col_cursor = 1;

            // write names
            for i in 1..=block.entries.len() {
                let pos_row = sec_height * (section_cursor - 1);

                let rg = block.entries[i - 1].range().to_string();
                worksheet.write_with_format(
                    (pos_row + i) as u32,
                    0,
                    rg.clone(),
                    format_of.get("name").unwrap(),
                )?;

                // record max length
                max_name_len = max(rg.len(), max_name_len);
            }

            for s in subs.iter() {
                // eprintln!("s = {:#?}", s.to_string());
                let pos_row = sec_height * (section_cursor - 1);

                // write position
                worksheet.write_with_format(
                    pos_row as u32,
                    col_cursor,
                    s.pos,
                    format_of.get("pos").unwrap(),
                )?;

                for i in 1..=seq_count {
                    let base = s.bases.chars().nth(i - 1).unwrap();
                    let occurred = if s.pattern == "unknown" {
                        '0'
                    } else {
                        s.pattern.chars().nth(i - 1).unwrap()
                    };

                    let base_color = if occurred == '1' {
                        let bg_idx = u32::from_str_radix(&s.pattern, 2).unwrap() % color_loop;
                        format!("sub_{}_{}", base, bg_idx)
                    } else {
                        format!("sub_{}_unknown", base)
                    };
                    let format = format_of.get(&base_color).unwrap();
                    worksheet.write_with_format(
                        (pos_row + i) as u32,
                        col_cursor,
                        base.to_string(),
                        format,
                    )?;
                }

                // increase column cursor
                col_cursor += 1;

                // wrap
                if col_cursor > wrap {
                    col_cursor = 1;
                    section_cursor += 1;
                }
            } // vars

            section_cursor += 1;
        } // block
    }

    worksheet.set_column_width(0, max_name_len as f64)?;
    for i in 1..=(wrap + 3) {
        worksheet.set_column_width(i, 1.6)?;
    }

    // Save the file to disk.
    workbook.save(outfile)?;

    Ok(())
}

fn create_formats() -> BTreeMap<String, Format> {
    let mut format_of: BTreeMap<String, Format> = BTreeMap::new();

    // species names
    format_of.insert(
        "name".to_string(),
        Format::new().set_font_name("Courier New").set_font_size(10),
    );

    // align positions of variations
    format_of.insert(
        "pos".to_string(),
        Format::new()
            .set_font_name("Courier New")
            .set_font_size(8)
            .set_align(FormatAlign::VerticalCenter)
            .set_align(FormatAlign::Center)
            .set_rotation(90),
    );

    // the standard Excel colors in the range 8..63

    // 15 colors
    let bg_colors: Vec<u32> = vec![
        0xC0C0C0, // Gray-25%, silver, 22
        0xFFFF99, // Light Yellow, 43
        0xCCFFCC, // Light Green, 42
        0xCCFFFF, // Lite Turquoise, 27
        0x99CCFF, // Pale Blue, 44
        0xCC99FF, // Lavender, 46
        0xFFCC99, // Tan, 47
        0x9999FF, // Periwinkle, 24
        0x33CCCC, // Aqua, 49
        0xFFCC00, // Gold, 51
        0xFF99CC, // Rose, 45
        0xFF9900, // Light Orange, 52
        0xFFFFCC, // Ivory, 26
        0xFF8080, // Coral, 29
        0xCCCCFF, // Ice Blue, 31
                  // 0x0066CC,       // Ocean Blue, 30
                  // 0xCCFFFF,       // Light Turquoise, again, 41
                  // 0x3366FF,       // Light Blue, 48
                  // 0x99CC00,       // Lime, 50
                  // 0x666699,       // Blue-Gray, 54
                  // 0x333399,       // Indigo, 62
    ];

    // font colors
    let sub_fc_of: BTreeMap<String, u32> = BTreeMap::from([
        ("A".to_string(), 0x003300), // Dark Green, 58
        ("C".to_string(), 0x000080), // Dark Blue, Navy, 18
        ("G".to_string(), 0x660066), // Dark Purple, 28
        ("T".to_string(), 0x800000), // Dark Red, Brown, 16
        ("N".to_string(), 0x000000), // Black, 8
        ("-".to_string(), 0x000000), // Black, 8
    ]);

    // sub _ base       _ color rotation
    // sub - font color - background color
    for fc in sub_fc_of.keys() {
        format_of.insert(
            format!("sub_{}_{}", fc, "unknown"),
            Format::new()
                .set_font_name("Courier New")
                .set_font_size(10)
                .set_align(FormatAlign::VerticalCenter)
                .set_align(FormatAlign::Center)
                .set_font_color(*sub_fc_of.get(fc).unwrap())
                .set_background_color(Color::White),
        );

        for i in 0..bg_colors.len() {
            let key = format!("sub_{}_{}", fc, i);
            format_of.insert(
                key,
                Format::new()
                    .set_font_name("Courier New")
                    .set_font_size(10)
                    .set_align(FormatAlign::VerticalCenter)
                    .set_align(FormatAlign::Center)
                    .set_font_color(*sub_fc_of.get(fc).unwrap())
                    .set_background_color(*bg_colors.get(i).unwrap()),
            );
        }
    }

    format_of.insert(
        "sub_-".to_string(),
        Format::new()
            .set_font_name("Courier New")
            .set_font_size(10)
            .set_align(FormatAlign::VerticalCenter)
            .set_align(FormatAlign::Center),
    );

    for i in 0..bg_colors.len() {
        let key = format!("indel_{}", i);
        format_of.insert(
            key,
            Format::new()
                .set_font_name("Courier New")
                .set_font_size(10)
                .set_align(FormatAlign::VerticalCenter)
                .set_align(FormatAlign::Center)
                .set_background_color(*bg_colors.get(i).unwrap()),
        );
    }

    format_of
}
