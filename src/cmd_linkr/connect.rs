use clap::*;
use intspan::*;
use petgraph::prelude::NodeIndex;
use petgraph::*;
use std::cmp;
use std::collections::{BTreeSet, HashMap};
use std::io::BufRead;

// Create clap subcommand arguments
pub fn make_subcommand<'a>() -> Command<'a> {
    Command::new("connect")
        .about("Connect bilateral links into multilateral ones")
        .after_help(
            "\
             <infiles> are bilateral link files without hit strands\
             ",
        )
        .arg(
            Arg::new("infiles")
                .help("Sets the input file to use")
                .required(true)
                .min_values(1)
                .index(1),
        )
        .arg(
            Arg::new("ratio")
                .long("ratio")
                .short('r')
                .takes_value(true)
                .default_value("0.9")
                .value_parser(value_parser!(f32))
                .help("Break links if length identities less than this ratio"),
        )
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .short('v')
                .help("Verbose mode"),
        )
        .arg(
            Arg::new("outfile")
                .short('o')
                .long("outfile")
                .takes_value(true)
                .default_value("stdout")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .help("Output filename. [stdout] for screen"),
        )
}

// command implementation
pub fn execute(args: &ArgMatches) -> anyhow::Result<()> {
    //----------------------------
    // Loading
    //----------------------------
    let ratio = *args.get_one::<f32>("ratio").unwrap();
    let is_verbose = args.contains_id("verbose");

    // all chromosomes stored in one graph
    let mut graph: Graph<String, String, Undirected, u32> = Graph::new_undirected();

    // cache ranges
    let mut range_of_part: HashMap<String, Range> = HashMap::new();

    // cache node indices
    // petgraph use NodeIndex to store and identify nodes
    let mut idx_of_part: HashMap<String, NodeIndex> = HashMap::new();

    for infile in args.get_many::<String>("infiles").unwrap() {
        if is_verbose {
            eprintln!("==> Loading {:#?}", infile);
        }

        let reader = reader(infile);
        for line in reader.lines().filter_map(|r| r.ok()) {
            build_range_of_part(&line, &mut range_of_part);

            let mut parts: Vec<String> = line
                .split('\t')
                .map(String::from)
                .filter(|part| range_of_part.contains_key(part))
                .collect();
            let count = parts.len();

            // make sure that all lines are bilateral links
            if count != 2 {
                continue;
            }

            // all ranges will be converted to positive strands
            // hit strands will be stored as edge weights
            let mut strands: BTreeSet<String> = BTreeSet::new();
            for i in 0..=1 {
                let mut range = range_of_part[&parts[i]].clone();
                strands.insert(range.strand().to_string());
                *range.strand_mut() = "+".to_string();
                parts[i] = range.to_string();
                build_range_of_part(&parts[i], &mut range_of_part);
            }

            let hit_strand = if strands.len() == 1 {
                "+".to_string()
            } else {
                "-".to_string()
            };

            // add ranges
            for part in &parts {
                if !idx_of_part.contains_key(part) {
                    let idx = graph.add_node(part.to_string());
                    idx_of_part.insert(part.to_string(), idx);

                    if is_verbose {
                        eprintln!("Add range {} as {:#?}", part, idx);
                    }
                }
            }

            // add edges
            if graph
                .find_edge(idx_of_part[&parts[0]], idx_of_part[&parts[1]])
                .is_some()
            {
                if is_verbose {
                    eprintln!("Edge exists, next");
                }
            } else {
                let edge =
                    graph.add_edge(idx_of_part[&parts[0]], idx_of_part[&parts[1]], hit_strand);
                if is_verbose {
                    eprintln!("    Add edge {}\t{} as {:#?}", parts[0], parts[1], edge);
                }
            }
        } // end of line
    } // end of file

    //----------------------------
    // Create cc and sort
    //----------------------------
    if is_verbose {
        eprintln!("==> Find connected components");
    }
    let mut cc_lines: Vec<String> = Vec::new();

    let scc: Vec<Vec<NodeIndex>> = petgraph::algo::tarjan_scc(&graph);
    for cc_indices in &scc {
        if cc_indices.len() < 2 {
            continue;
        }

        // connected parts
        let part_list = cc_indices
            .iter()
            .map(|idx| graph.node_weight(*idx).unwrap().clone())
            .collect::<Vec<String>>();
        cc_lines.push(part_list.join("\t"));
    }
    cc_lines = sort_links(&cc_lines);
    if is_verbose {
        eprintln!("==> Total {:#?} connected components", cc_lines.len());
    }

    //----------------------------
    // Change strands of ranges based on first range in cc
    //----------------------------
    if is_verbose {
        eprintln!("==> Change strands");
    }
    // no edge weights
    let mut new_graph: Graph<String, (), Undirected, u32> = Graph::new_undirected();
    let mut new_idx_of_part: HashMap<String, NodeIndex> = HashMap::new();
    for line in &cc_lines {
        let parts: Vec<String> = line.split('\t').map(String::from).collect();
        let count = parts.len();
        if is_verbose {
            eprintln!("Copy number of this cc is {}", count);
        }

        // empty strands
        let mut ranges = parts
            .iter()
            .map(|part| {
                let mut range = Range::from_str(part);
                *range.strand_mut() = "".to_string();
                range
            })
            .collect::<Vec<Range>>();

        // set first node to positive strand
        *ranges[0].strand_mut() = "+".to_string();

        // assign strands to other nodes
        let mut assigned: BTreeSet<usize> = BTreeSet::new();
        assigned.insert(0);
        let mut unhandled: BTreeSet<usize> = (1..count).collect();
        //        eprintln!("unhandled = {:#?}", unhandled);

        let mut edges = vec![];
        while assigned.len() < count {
            for i in assigned.iter().cloned().collect::<Vec<usize>>() {
                for j in unhandled.iter().cloned().collect::<Vec<usize>>() {
                    // not connected in old graph
                    let edge = graph.find_edge(idx_of_part[&parts[i]], idx_of_part[&parts[j]]);
                    if edge.is_none() {
                        continue;
                    }

                    // assign strand
                    let hit_strand = graph.edge_weight(edge.unwrap()).unwrap();
                    if hit_strand == "-" {
                        if is_verbose {
                            eprint!(
                                "    Based on {}, change strand from {}",
                                ranges[i], ranges[j]
                            );
                        }
                        if ranges[i].strand() == "-" {
                            *ranges[j].strand_mut() = "+".to_string();
                        } else {
                            *ranges[j].strand_mut() = "-".to_string();
                        }
                        if is_verbose {
                            eprintln!(" to {}", ranges[j]);
                        }
                    } else {
                        *ranges[j].strand_mut() = ranges[i].strand().to_string();
                    }
                    let j_copy = unhandled.take(&j).unwrap();
                    assigned.insert(j_copy);

                    // break bad links
                    let size_i = ranges[i].intspan().cardinality();
                    let size_j = ranges[j].intspan().cardinality();
                    let size_min = cmp::min(size_i, size_j);
                    let size_max = cmp::max(size_i, size_j);
                    let diff_ratio = size_min as f32 / size_max as f32;

                    if diff_ratio < ratio {
                        if is_verbose {
                            eprintln!("    Break edge between {}\t{}", parts[i], parts[j]);
                            eprintln!(
                                "    Ratio[{}]\tMin [{}]\tMax[{}]",
                                diff_ratio, size_min, size_max
                            );
                        }
                    } else {
                        edges.push((i, j));
                    }
                }
            }
        } // end of assigning

        for (i, j) in edges {
            let part_i = ranges[i].to_string();
            let part_j = ranges[j].to_string();

            for part in &vec![part_i.to_string(), part_j.to_string()] {
                if !new_idx_of_part.contains_key(part) {
                    let idx = new_graph.add_node(part.to_string());
                    new_idx_of_part.insert(part.to_string(), idx);
                }
            }
            new_graph.add_edge(new_idx_of_part[&part_i], new_idx_of_part[&part_j], ());
        }
    }

    //----------------------------
    // Recreate cc
    //----------------------------
    if is_verbose {
        eprintln!("==> Recreate connected components");
    }
    let mut out_lines: Vec<String> = Vec::new();

    let scc: Vec<Vec<NodeIndex>> = petgraph::algo::tarjan_scc(&new_graph);
    for cc_indices in &scc {
        if cc_indices.len() < 2 {
            continue;
        }

        // connected parts
        let part_list = cc_indices
            .iter()
            .map(|idx| new_graph.node_weight(*idx).unwrap().clone())
            .collect::<Vec<String>>();
        out_lines.push(part_list.join("\t"));
    }
    out_lines = sort_links(&out_lines);

    //----------------------------
    // Output
    //----------------------------
    write_lines(
        args.get_one::<String>("outfile").unwrap(),
        &out_lines.iter().map(AsRef::as_ref).collect(),
    )?;

    Ok(())
}
