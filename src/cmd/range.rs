use intspan::*;
use clap::*;
use std::io::BufRead;

// Create clap subcommand arguments
pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("range")
        .about("Convert runlist file to ranges file")
        .arg(
            Arg::with_name("runlist")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("ranges")
                .help("Sets the input file to use")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("op")
                .long("op")
                .takes_value(true)
                .default_value("overlap")
                .empty_values(false)
                .help("operations: overlap, non-overlap or superset"),
        )
        .arg(
            Arg::with_name("outfile")
                .short("o")
                .long("outfile")
                .takes_value(true)
                .default_value("stdout")
                .empty_values(false)
                .help("Output filename. [stdout] for screen"),
        )
}

// command implementation
pub fn execute(args: &ArgMatches) {
    //----------------------------
    // Loading
    //----------------------------
    let yaml = read_yaml(args.value_of("runlist").unwrap());
    let set = yaml2set(&yaml);

    let reader = reader(args.value_of("ranges").unwrap());
    let mut writer = writer(args.value_of("outfile").unwrap());

    let op = args.value_of("op").unwrap();

    //----------------------------
    // Operating
    //----------------------------
    for line in reader.lines().filter_map(|r| r.ok()) {
        let range = Range::from_str(line.clone());
        if range.start() == &0 {
            continue;
        }
        let chr = range.chr();
        let mut intspan = IntSpan::new();
        intspan.add_pair(range.start().clone(), range.end().clone());

        //----------------------------
        // Output
        //----------------------------
        match op {
            "overlap" => {
                if set.contains_key(chr) && !set.get(chr).unwrap().intersect(&intspan).is_empty() {
                    writer.write_all((line + "\n").as_ref());
                }
            }
            "non-overlap" => {
                if set.contains_key(chr) {
                    if set.get(chr).unwrap().intersect(&intspan).is_empty() {
                        writer.write_all((line + "\n").as_ref());
                    }
                } else {
                    writer.write_all((line + "\n").as_ref());
                }
            }
            "superset" => {
                if set.contains_key(chr) && set.get(chr).unwrap().superset(&intspan) {
                    writer.write_all((line + "\n").as_ref());
                }
            }
            _ => panic!("Invalid Range Op"),
        };
    }
}
