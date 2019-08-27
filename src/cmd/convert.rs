use intspan::*;
use clap::*;

// Create clap subcommand arguments
pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("convert")
        .about("Convert runlist file to ranges file")
        .arg(
            Arg::with_name("infiles")
                .help("Sets the input file to use")
                .required(true)
                .min_values(1)
                .index(1),
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
    let mut writer = writer(args.value_of("outfile").unwrap());

    //----------------------------
    // Operating
    //----------------------------
    for infile in args.values_of("infiles").unwrap() {
        let yaml = read_yaml(infile);
        let set_of = yaml2set_m(&yaml);

        for set in set_of.values() {
            for chr in set.keys() {
                let intspan = set.get(chr).unwrap();
                let span_size = intspan.span_size();
                let ranges = intspan.ranges();
                for i in 0..span_size {
                    let lower = *ranges.get(i * 2).unwrap();
                    let upper = *ranges.get(i * 2 + 1).unwrap();

                    //----------------------------
                    // Output
                    //----------------------------
                    if lower == upper {
                        writer.write_all(format!("{}:{}\n", chr, lower).as_ref());
                    } else {
                        writer.write_all(format!("{}:{}-{}\n", chr, lower, upper).as_ref());
                    }
                }
            }
        }
    }
}
