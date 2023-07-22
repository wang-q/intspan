use clap::*;
use cmd_lib::*;
use std::collections::BTreeMap;
use std::{env, fs};

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("pl-p2m")
        .about("Pipeline - pairwise alignments to multiple alignments")
        .after_help(
            r###"
* <infiles> are paths to block fasta files, .fas.gz is supported
    * infile == stdin means reading from STDIN

* The pl-* subcommands
    * The default --outdir is `PL-*`, not `.`
    * There is no option to output to the screen

* All operations are based on the *first* species name of the *first* input file

* This pipeline depends on two binaries, `fasr` and `spanr`

"###,
        )
        .arg(
            Arg::new("infiles")
                .required(true)
                .num_args(2..)
                .index(1)
                .help("Set the input files to use"),
        )
        .arg(
            Arg::new("outdir")
                .short('o')
                .long("outdir")
                .num_args(1)
                .default_value("PL-p2m")
                .help("Output location"),
        )
}

// command implementation
pub fn execute(args: &ArgMatches) -> anyhow::Result<()> {
    //----------------------------
    // Args
    //----------------------------
    let outdir = args.get_one::<String>("outdir").unwrap();
    fs::create_dir_all(outdir)?;

    let curdir = env::current_dir()?;
    let fasr = env::current_exe().unwrap().display().to_string();

    use_builtin_cmd!(echo, cat);
    run_cmd!(echo "==> Paths")?;
    run_cmd!(echo "    \"fasr\"   = ${fasr}")?;
    run_cmd!(echo "    \"curdir\" = ${curdir}")?;
    run_cmd!(echo "    \"outdir\" = ${outdir}")?;

    // basename => [abs_path, .json, .slice.fas]
    let mut info_of: BTreeMap<String, Vec<String>> = BTreeMap::new();

    //----------------------------
    // Operating
    //----------------------------
    run_cmd!(echo "==> Basenames and absolute paths")?;
    for infile in args.get_many::<String>("infiles").unwrap() {
        let basename = intspan::basename(infile)?;
        let absolute = intspan::absolute_path(infile)
            .unwrap()
            .display()
            .to_string();

        info_of.insert(basename.to_string(), vec![absolute.to_string()]);
    }

    run_cmd!(echo "==> Switch to outdir")?;
    env::set_current_dir(outdir)?;

    run_cmd!(echo "==> fasr name - first")?;
    let mut target_name = "".to_string();
    {
        let infile = info_of.first_key_value().unwrap().1.get(0).unwrap();
        run_cmd!(
            ${fasr} name ${infile} -o name.first.lst
        )?;
        target_name = run_fun!(cat name.first.lst)
            .unwrap()
            .split('\n')
            .next()
            .unwrap()
            .to_string();
        run_cmd!(echo "    \"target_name\" = ${target_name}")?;
    }

    run_cmd!(echo "==> fasr cover")?;
    for (basename, info) in info_of.iter_mut() {
        let infile = info.get(0).unwrap();
        let outfile = format!("{}.json", basename);
        run_cmd!(${fasr} cover ${infile} --trim 10 --name ${target_name} -o ${outfile})?;

        info.push(outfile.to_string());
    }

    run_cmd!(echo "==> spanr compare")?;
    {
        let infiles: Vec<String> = info_of
            .iter()
            .map(|e| e.1.get(1).unwrap().to_string())
            .collect();
        run_cmd!(
            spanr compare --op intersect $[infiles] |
                spanr span stdin --op excise -n 10 -o intersect.json
        )?;
        run_cmd!(
            spanr merge $[infiles] intersect.json -o merge.json
        )?;
    }

    run_cmd!(echo "==> fasr slice")?;
    for (basename, info) in info_of.iter_mut() {
        let infile = info.get(0).unwrap();
        let outfile = format!("{}.slice.fas", basename);
        run_cmd!(${fasr} slice intersect.json ${infile} --name ${target_name} -o ${outfile})?;

        info.push(outfile.to_string());
    }

    run_cmd!(echo "==> fasr join")?;
    {
        let infiles: Vec<String> = info_of
            .iter()
            .map(|e| e.1.get(2).unwrap().to_string())
            .collect();
        run_cmd!(
            ${fasr} join $[infiles] --name ${target_name} -o join.raw.fas
        )?;
    }

    run_cmd!(echo "==> fasr name && fasr subset")?;
    {
        run_cmd!(
            ${fasr} name join.raw.fas -o name.lst
        )?;
        run_cmd!(
            ${fasr} subset name.lst join.raw.fas --required -o join.subset.fas
        )?;
    }

    // eprintln!("info_of = {:#?}", info_of);

    //----------------------------
    // Done
    //----------------------------
    env::set_current_dir(&curdir)?;

    Ok(())
}

// fn pause() {
//     let mut stdin = io::stdin();
//     let mut stdout = io::stdout();
//
//     // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
//     write!(stdout, "Press any key to continue...").unwrap();
//     stdout.flush().unwrap();
//
//     // Read a single byte and discard
//     let _ = stdin.read(&mut [0u8]).unwrap();
// }
