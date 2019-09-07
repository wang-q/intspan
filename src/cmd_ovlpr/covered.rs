use clap::*;
use indexmap::IndexSet;
use intspan::*;
use std::collections::{BTreeMap, HashMap, HashSet};
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
            Arg::with_name("paf")
                .long("paf")
                .help("PAF as input format"),
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
pub fn execute(args: &ArgMatches) -> std::result::Result<(), std::io::Error> {
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
    let is_paf = args.is_present("paf");
    let is_longest = args.is_present("longest");
    let is_base = args.is_present("base");
    let is_mean = args.is_present("mean");

    // seq_name => tier_of => IntSpan
    let mut res: HashMap<String, Coverage> = HashMap::new();
    let mut index_of: IndexSet<String> = IndexSet::new();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();

    for infile in args.values_of("infiles").unwrap() {
        let reader = reader(infile);
        for line in reader.lines().filter_map(|r| r.ok()) {
            let ovlp = if is_paf {
                Overlap::from_paf(&line)
            } else {
                Overlap::new(&line)
            };
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

            // skip duplicated overlaps, i.e., f -> g and g -> f
            let (f_idx, _) = index_of.insert_full(f_id.clone());
            let (g_idx, _) = index_of.insert_full(g_id.clone());
            let tup = (f_idx.min(g_idx), f_idx.max(g_idx));
            // If the set did not have this value present, true is returned.
            let not_seen = seen.insert(tup);
            if !not_seen {
                continue;
            }

            // first
            if !res.contains_key(f_id) {
                let tiers = Coverage::new_len(coverage, *ovlp.f_len());
                res.insert(f_id.clone(), tiers);
            }
            res.entry(f_id.to_string())
                .and_modify(|e| e.bump(ovlp.f_begin().clone(), ovlp.f_end().clone()));

            // second
            if !res.contains_key(g_id) {
                let tiers = Coverage::new_len(coverage, *ovlp.g_len());
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

        if is_base {
            let tiers = res.get(key).unwrap().uniq_tiers();
            out_line = base_lines(key, &tiers);
        } else if is_mean {
            let tiers = res.get(key).unwrap().uniq_tiers();
            out_line = mean_line(key, &tiers);
        } else {
            let intspan = res.get(key).unwrap().max_tier();

            if !is_longest {
                out_line = format!("{}:{}", key, intspan.to_string());
            } else {
                out_line = longest_line(key, &intspan);
            }
        }

        writer.write_all((out_line + "\n").as_ref())?;
    }

    Ok(())
}

fn base_lines(key: &str, tiers: &BTreeMap<i32, IntSpan>) -> String {
    let mut basecovs: HashMap<i32, i32> = HashMap::new();
    let max_tier = tiers.keys().max().unwrap();
    for i in 0..=*max_tier {
        for pos in tiers[&i].elements() {
            basecovs.insert(pos, i);
        }
    }

    let mut sorted: Vec<i32> = basecovs.keys().copied().collect();
    sorted.sort();

    let mut out_lines: Vec<String> = vec![];
    for pos in sorted {
        let line = format!("{}\t{}\t{}", key, pos - 1, basecovs[&pos]);
        out_lines.push(line);
    }

    out_lines.join("\n")
}

fn mean_line(key: &str, tiers: &BTreeMap<i32, IntSpan>) -> String {
    let total_len = tiers[&-1].cardinality();
    let max_tier = tiers.keys().max().unwrap();
    let mut sum = 0;
    for i in 0..=*max_tier {
        sum += i * tiers[&i].cardinality();
    }
    let mean_cov = sum as f32 / total_len as f32;

    format!("{}\t{}\t{:.1}", key, total_len, mean_cov)
}

fn longest_line(key: &str, intspan: &IntSpan) -> String {
    let ranges = intspan.ranges();

    let mut sizes: Vec<i32> = Vec::new();
    for i in 0..intspan.span_size() {
        let size = ranges[i * 2 + 1] - ranges[i * 2] + 1;
        sizes.push(size);
    }

    let mut max_i = 0;
    for i in 0..intspan.span_size() {
        let size = sizes[i];
        if size > sizes[max_i] {
            max_i = i;
        }
    }

    let mut longest = IntSpan::new();
    longest.add_pair(ranges[max_i * 2], ranges[max_i * 2 + 1]);

    format!("{}:{}", key, longest.to_string())
}
