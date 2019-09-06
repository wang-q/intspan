use clap::*;
use intspan::*;
use std::collections::HashMap;
use std::io::BufRead;

// Create clap subcommand arguments
pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("covered")
        .about("Covered regions from .ovlp.tsv files")
        .arg(
            Arg::with_name("infiles")
                .help("Sets the input file to use")
                .required(true)
                .min_values(1)
                .index(1),
        )
        .arg(
            Arg::with_name("coverage")
                .help("minimal coverage")
                .long("coverage")
                .short("c")
                .takes_value(true)
                .default_value("3")
                .empty_values(false),
        )
        .arg(
            Arg::with_name("len")
                .help("minimal length of overlaps")
                .long("len")
                .short("l")
                .takes_value(true)
                .default_value("1000")
                .empty_values(false),
        )
        .arg(
            Arg::with_name("idt")
                .help("minimal identities of overlaps")
                .long("idt")
                .short("i")
                .takes_value(true)
                .default_value("0.0")
                .empty_values(false),
        )
        .arg(
            Arg::with_name("longest")
                .long("longest")
                .help("only keep the longest span"),
        )
        .arg(
            Arg::with_name("base")
                .long("base")
                .help("per base coverage"),
        )
        .arg(Arg::with_name("mean").long("mean").help("mean coverage"))
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

    let coverage: i32 = value_t!(args.value_of("coverage"), i32).unwrap_or_else(|e| {
        eprintln!("Need a integer for --coverage\n{}", e);
        std::process::exit(1)
    });
    let min_len: i32 = value_t!(args.value_of("len"), i32).unwrap_or_else(|e| {
        eprintln!("Need a integer for --len\n{}", e);
        std::process::exit(1)
    });
    let min_idt: f32 = value_t!(args.value_of("idt"), f32).unwrap_or_else(|e| {
        eprintln!("Need a integer for --idt\n{}", e);
        std::process::exit(1)
    });
    let is_longest = args.is_present("longest");
    let is_base = args.is_present("base");
    let is_mean = args.is_present("mean");

    // seq_name => tier_of => IntSpan
    let mut res: HashMap<String, Coverage> = HashMap::new();

    for infile in args.values_of("infiles").unwrap() {
        let reader = reader(infile);
        for line in reader.lines().filter_map(|r| r.ok()) {
            let ovlp = Overlap::new(&line);
            let f_id = ovlp.f_id();
            let g_id = ovlp.g_id();

            // ignore self overlapping
            if f_id == g_id {
                continue;
            }

            // ignore poor overlaps
            if *ovlp.len() < min_len {
                continue;
            }
            if *ovlp.idt() < min_idt {
                continue;
            }

            //            if !ovlp.is_valid() {
            //                continue;
            //            }

            // first
            if !res.contains_key(f_id) {
                let tiers = Coverage::new(coverage);
                res.insert(f_id.clone(), tiers);
            }

            res.entry(f_id.to_string())
                .and_modify(|e| e.bump(ovlp.f_begin().clone(), ovlp.f_end().clone()));

            // second
            if !res.contains_key(g_id) {
                let tiers = Coverage::new(coverage);
                res.insert(g_id.clone(), tiers);
            }

            res.entry(g_id.to_string())
                .and_modify(|e| e.bump(ovlp.g_begin().clone(), ovlp.g_end().clone()));
        }
    }

    //----------------------------
    // Output
    //----------------------------
    let mut keys = res.keys().map(|k| k.to_string()).collect::<Vec<String>>();
    keys.sort();

    for key in &keys {
        let mut out_line = String::new();

        let intspan = res.get(key).unwrap().max_tier();

        if !is_longest {
            out_line = format!("{}:{}", key, intspan.to_string());
        } else if intspan.span_size() <= 1 {
            out_line = format!("{}:{}", key, intspan.to_string());
        } else {
            let ranges = intspan.ranges();

            let mut sizes: Vec<i32> = Vec::new();
            for i in 0..intspan.span_size() {
                let size = ranges[i * 2 + 1] - ranges[i * 2] + 1;
                sizes.push(size);
            }

            let mut max_idx = 0;
            for i in 0..intspan.span_size() {
                let size = sizes[i];
                if size > sizes[i] {
                    max_idx = i;
                }
            }

            let mut longest = IntSpan::new();
            longest.add_pair(ranges[max_idx * 2], ranges[max_idx * 2 + 1]);
            out_line = format!("{}:{}", key, longest.to_string());
        }

        writer.write_all((out_line + "\n").as_ref());
    }
    //    let mut set: BTreeMap<String, IntSpan> = BTreeMap::new();
    //    for key in res.keys() {
    //        set.insert(key.to_string(), res.get(key).unwrap().max_tier());
    //    }
    //    let out_yaml = set2yaml(&set);
    //    write_yaml(args.value_of("outfile").unwrap(), &out_yaml);
}
