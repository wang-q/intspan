use clap::*;
use intspan::*;
use petgraph::prelude::NodeIndex;
use petgraph::*;
use std::collections::{HashMap, HashSet};
use std::io::BufRead;

// Create clap subcommand arguments
pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("merge")
        .about("Merge overlapped ranges via overlapping graph")
        .arg(
            Arg::with_name("infiles")
                .help("Sets the input file to use")
                .required(true)
                .min_values(1)
                .index(1),
        )
        .arg(
            Arg::with_name("coverage")
                .long("coverage")
                .short("c")
                .takes_value(true)
                .default_value("0.95")
                .empty_values(false)
                .help("When larger than this ratio, merge ranges"),
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
    let coverage: f32 = value_t!(args.value_of("coverage"), f32).unwrap_or_else(|e| {
        eprintln!("Need a float for --coverage\n{}", e);
        std::process::exit(1)
    });
    let is_verbose = args.is_present("verbose");

    // store graph separately by chromosomes
    // petgraph use NodeIndex to store and identify nodes
    let mut graph_of_chr: HashMap<String, Graph<String, _, Undirected>> = HashMap::new();

    // cache ranges
    let mut range_of_part: HashMap<String, Range> = HashMap::new();
    // cache node indices
    let mut idx_of_part: HashMap<String, NodeIndex> = HashMap::new();

    // all chromosomes
    let mut chrs: HashSet<String> = HashSet::new();

    for infile in args.values_of("infiles").unwrap() {
        let reader = reader(infile);
        for line in reader.lines().filter_map(|r| r.ok()) {
            for part in line.split('\t') {
                let range = Range::from_str(part);
                if !range.is_valid() {
                    continue;
                }

                if range_of_part.contains_key(part) {
                    continue;
                }

                let chr = range.chr();
                if !graph_of_chr.contains_key(chr) {
                    graph_of_chr.insert(chr.to_string(), Graph::new_undirected());
                }

                chrs.insert(chr.to_string());

                let idx = graph_of_chr
                    .get_mut(chr)
                    .unwrap()
                    .add_node(part.to_string());
                idx_of_part.insert(part.to_string(), idx);

                range_of_part.insert(part.to_string(), range);
            }
        } // end of line
    } // end of file
    let mut chrs = chrs.into_iter().collect::<Vec<String>>();
    chrs.sort();

    //----------------------------
    // Checking coverages
    //----------------------------
    for chr in &chrs {
        if is_verbose {
            eprintln!("Chromosome {}", chr);
        }

        let mut graph = graph_of_chr.get_mut(chr).unwrap();
        let indices = graph.node_indices().collect::<Vec<NodeIndex>>();

        for i in 0..indices.len() {
            let node_i = graph.node_weight(indices[i]).unwrap();
            let intspan_i = range_of_part[node_i].intspan();
            if is_verbose {
                eprintln!("    Range {}/{}\t{}", i, indices.len(), node_i);
            }

            for j in i + 1..indices.len() {
                let node_j = graph.node_weight(indices[j]).unwrap();
                let intspan_j = range_of_part[node_j].intspan();

                let intersect = intspan_i.intersect(&intspan_j);
                if !intersect.is_empty() {
                    let coverage_i =
                        intersect.cardinality() as f32 / intspan_i.cardinality() as f32;
                    let coverage_j =
                        intersect.cardinality() as f32 / intspan_j.cardinality() as f32;

                    if coverage_i >= coverage && coverage_j >= coverage {
                        if is_verbose {
                            eprintln!(
                                "        Merge with Range {}/{}\t{}",
                                j,
                                indices.len(),
                                node_j
                            );
                        }
                        graph.add_edge(indices[i], indices[j], ());
                    }
                }
            }
        }
    }

    //----------------------------
    // Merging
    //----------------------------
    let mut out_lines: Vec<String> = Vec::new();
    for chr in &chrs {
        let graph = graph_of_chr.get(chr).unwrap();

        let scc : Vec<Vec<NodeIndex>> = petgraph::algo::tarjan_scc(graph);
        for connected_indices in &scc {
            if connected_indices.len() < 2 {
                continue;
            }

            if is_verbose {
                eprintln!("Chromosome {}: Merge {} ranges", chr, connected_indices.len());
            }

            // connected ranges
            let mut part_list = connected_indices.into_iter().map(|idx|
                graph.node_weight(*idx).unwrap().clone()
            ).collect::<Vec<String>>();
            part_list.sort();

            // collect info for merged range
            let mut intspan = IntSpan::new();
            for part in &part_list {
                let range = range_of_part.get(part).unwrap();
                intspan.merge(&range.intspan());
            }

            // create merged range
            let merged : String = format!("{}(+):{}", chr, intspan.to_string());

            for part in &part_list {
                if *part == merged {
                    continue;
                }

                let out_line = format!("{}\t{}", part, merged);
                if is_verbose {
                    eprintln!("{}", out_line);
                }
                out_lines.push(out_line);
            }
        }
    }

    //----------------------------
    // Output
    //----------------------------
    write_lines(
        args.value_of("outfile").unwrap(),
        &out_lines.iter().map(AsRef::as_ref).collect(),
    );
}
