use clap::*;
use intspan::*;
use petgraph::prelude::NodeIndex;
use petgraph::*;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::io::BufRead;

// Create clap subcommand arguments
pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("clean")
        .about("Replace ranges within links, incorporate hit strands and remove nested links")
        .after_help(
            "\
             <infiles> are bilateral links files, with or without hit strands\
             ",
        )
        .arg(
            Arg::with_name("infiles")
                .help("Sets the input file to use")
                .required(true)
                .min_values(1)
                .index(1),
        )
        .arg(
            Arg::with_name("replace")
                .long("replace")
                .short("r")
                .takes_value(true)
                .empty_values(false)
                .help("Two-column tsv file, normally produced by command merge"),
        )
        .arg(
            Arg::with_name("bundle")
                .long("bundle")
                .short("b")
                .takes_value(true)
                .default_value("0")
                .empty_values(false)
                .help("Bundle overlapped links. This value is the overlapping size. Suggested value is [500]"),
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .help("Verbose mode"),
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
    let bundle: i32 = value_t!(args.value_of("bundle"), i32).unwrap_or_else(|e| {
        eprintln!("Need a integer for --bundle\n{}", e);
        std::process::exit(1)
    });
    let is_verbose = args.is_present("verbose");

    // cache ranges
    let mut range_of_part: HashMap<String, Range> = HashMap::new();

    //----------------------------
    // Load replaces
    //----------------------------
    let mut replaces: HashMap<String, String> = HashMap::new();
    if args.is_present("replace") {
        if is_verbose {
            eprintln!("==> Load replaces");
        }
        for line in read_lines(args.value_of("replace").unwrap()) {
            let fields: Vec<&str> = line.split('\t').collect();
            if fields.len() == 2 {
                replaces.insert(fields[0].to_string(), fields[1].to_string());
            }
        }
    }

    //----------------------------
    // Replacing and incorporating strands
    //----------------------------
    if is_verbose {
        eprintln!("==> Replacing and incorporating strands");
    }

    let mut line_set: BTreeSet<String> = BTreeSet::new();
    for infile in args.values_of("infiles").unwrap() {
        let reader = reader(infile);
        for line in reader.lines().filter_map(|r| r.ok()) {
            build_range_of_part(&line, &mut range_of_part);
            let parts: Vec<&str> = line.split('\t').collect();
            let count = parts.len();

            // make sure that all lines are bilateral links
            if !(count == 2 || count == 3) {
                continue;
            }
            if !range_of_part.contains_key(parts[0]) {
                continue;
            }
            if !range_of_part.contains_key(parts[1]) {
                continue;
            }

            // replacing

            // incorporating strands
            let mut strands: BTreeSet<String> = BTreeSet::new();
            if count == 3 {
                if parts[2] == "+" || parts[2] == "-" {
                    strands.insert(parts[2].to_string());
                }
            }

            for i in &[0, 1] {
                strands.insert(range_of_part[parts[*i as usize]].strand().to_string());
            }
            //            eprintln!("strands = {:#?}", strands);

            let mut range_0 = range_of_part[parts[0]].clone();
            let mut range_1 = range_of_part[parts[1]].clone();

            // skip identical ranges
            if range_0.chr() == range_1.chr()
                && range_0.start() == range_1.start()
                && range_0.end() == range_1.end()
            {
                continue;
            }

            if strands.len() == 1 {
                *range_0.strand_mut() = "+".to_string();
                *range_1.strand_mut() = "+".to_string();
            } else {
                *range_0.strand_mut() = "+".to_string();
                *range_1.strand_mut() = "-".to_string();
            }

            let new_line = format!("{}\t{}", range_0.to_string(), range_1.to_string());
            build_range_of_part(&new_line, &mut range_of_part);
            line_set.insert(new_line);
        }
    }

    //----------------------------
    // Remove nested links
    //----------------------------
    // now all lines (links) are without hit strands
    let mut lines = line_set
        .iter()
        .map(String::to_string)
        .collect::<Vec<String>>();
    lines = sort_links(&lines);
    let mut is_nested = true;
    while is_nested {
        if is_verbose {
            eprintln!("==> Remove nested links");
        }

        let mut to_remove: HashSet<String> = HashSet::new();
        let chr_pairs = lines
            .iter()
            .map(|line| {
                let parts: Vec<&str> = line.split('\t').collect();
                format!(
                    "{}:{}",
                    range_of_part[parts[0]].chr(),
                    range_of_part[parts[1]].chr()
                )
            })
            .collect::<Vec<String>>();

        for i in 0..lines.len() {
            let cur_pair = &chr_pairs[i];
            let rest_idx: Vec<usize> = (i + 1..lines.len())
                .filter(|key| chr_pairs[*key] == *cur_pair)
                .collect();

            for j in rest_idx {
                let line_i = &lines[i];
                let parts_i: Vec<&str> = line_i.split('\t').collect();

                let line_j = &lines[j];
                let parts_j: Vec<&str> = line_j.split('\t').collect();

                let intspan_0_i = range_of_part[parts_i[0]].intspan();
                let intspan_1_i = range_of_part[parts_i[1]].intspan();

                let intspan_0_j = range_of_part[parts_j[0]].intspan();
                let intspan_1_j = range_of_part[parts_j[1]].intspan();

                if intspan_0_i.superset(&intspan_0_j) && intspan_1_i.superset(&intspan_1_j) {
                    to_remove.insert(line_j.to_string());
                } else if intspan_0_j.superset(&intspan_0_i) && intspan_1_j.superset(&intspan_1_i) {
                    to_remove.insert(line_i.to_string());
                }
            }
        }

        lines = lines
            .iter()
            .filter(|key| !to_remove.contains(*key))
            .map(String::to_string)
            .collect::<Vec<String>>();
        is_nested = !to_remove.is_empty();
    }
    lines = sort_links(&lines);

    //----------------------------
    // Bundle links
    //----------------------------
    if bundle != 0 {
        if is_verbose {
            eprintln!("==> Bundle overlapped links");
        }

        let chr_strand_pairs = lines
            .iter()
            .map(|line| {
                let parts: Vec<&str> = line.split('\t').collect();
                format!(
                    "{}:{}:{}:{}",
                    range_of_part[parts[0]].chr(),
                    range_of_part[parts[0]].strand(),
                    range_of_part[parts[1]].chr(),
                    range_of_part[parts[1]].strand(),
                )
            })
            .collect::<Vec<String>>();

        let mut graph: Graph<String, (), Undirected> = Graph::new_undirected();
        // cache node indices
        let mut idx_of_line: HashMap<String, NodeIndex> = HashMap::new();

        for i in 0..lines.len() {
            let cur_pair = &chr_strand_pairs[i];
            let rest_idx: Vec<usize> = (i + 1..lines.len())
                .filter(|key| chr_strand_pairs[*key] == *cur_pair)
                .collect();

            for j in rest_idx {
                let line_i = &lines[i];
                let parts_i: Vec<&str> = line_i.split('\t').collect();

                let line_j = &lines[j];
                let parts_j: Vec<&str> = line_j.split('\t').collect();

                if !idx_of_line.contains_key(line_i) {
                    let idx = graph.add_node(line_i.to_string());
                    idx_of_line.insert(line_i.to_string(), idx);
                }
                if !idx_of_line.contains_key(line_j) {
                    let idx = graph.add_node(line_j.to_string());
                    idx_of_line.insert(line_j.to_string(), idx);
                }

                let intspan_0_i = range_of_part[parts_i[0]].intspan();
                let intspan_1_i = range_of_part[parts_i[1]].intspan();

                let intspan_0_j = range_of_part[parts_j[0]].intspan();
                let intspan_1_j = range_of_part[parts_j[1]].intspan();

                if intspan_0_i.intersect(&intspan_0_j).cardinality() >= bundle
                    && intspan_1_i.intersect(&intspan_1_j).cardinality() >= bundle
                {
                    graph.add_edge(idx_of_line[line_i], idx_of_line[line_j], ());
                }
            }
        }

        // bundle connected lines
        let scc: Vec<Vec<NodeIndex>> = petgraph::algo::tarjan_scc(&graph);
        for connected_indices in &scc {
            if connected_indices.len() < 2 {
                continue;
            }

            if is_verbose {
                eprintln!("Merge {} lines", connected_indices.len());
            }

            // connected lines
            let mut line_list = connected_indices
                .into_iter()
                .map(|idx| graph.node_weight(*idx).unwrap().clone())
                .collect::<Vec<String>>();
            line_list.sort();
            if is_verbose {
                eprintln!("line_list = {:#?}", line_list);
            }

            let mut merged_ranges: Vec<String> = Vec::new();
            for i in &[0, 1] {
                let mut chr = "".to_string();
                let mut strand = "".to_string();
                let mut intspan = IntSpan::new();

                for line in &line_list {
                    // remove lines to be merged
                    lines = lines
                        .iter()
                        .filter(|key| *key != line)
                        .map(String::to_string)
                        .collect::<Vec<String>>();

                    let parts: Vec<&str> = line.split('\t').collect();
                    let range = range_of_part.get(parts[*i as usize]).unwrap();
                    chr = range.chr().to_string();
                    strand = range.strand().to_string();
                    intspan.merge(&range.intspan());
                }

                let merged: String =
                    format!("{}({}):{}-{}", chr, strand, intspan.min(), intspan.max());
                merged_ranges.push(merged);
            }

            let new_line = merged_ranges.join("\t");
            build_range_of_part(&new_line, &mut range_of_part);
            if is_verbose {
                eprintln!("    To {}", new_line);
            }
            lines.push(new_line);
        }

        lines = sort_links(&lines);
    }

    //----------------------------
    // Output
    //----------------------------
    write_lines(
        args.value_of("outfile").unwrap(),
        &lines.iter().map(AsRef::as_ref).collect(),
    );
}
