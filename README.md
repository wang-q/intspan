# intspan

[![Build](https://github.com/wang-q/intspan/actions/workflows/build.yml/badge.svg)](https://github.com/wang-q/intspan/actions)
[![Windows build status](https://ci.appveyor.com/api/projects/status/github/wang-q/intspan?svg=true)](https://ci.appveyor.com/project/wang-q/intspan)
[![codecov](https://codecov.io/gh/wang-q/intspan/branch/master/graph/badge.svg?token=m8OIcyvuGr)](https://codecov.io/gh/wang-q/intspan)
[![Crates.io](https://img.shields.io/crates/v/intspan.svg)](https://crates.io/crates/intspan)
[![license](https://img.shields.io/github/license/wang-q/intspan)](https://github.com//wang-q/intspan)
[![Lines of code](https://tokei.rs/b1/github/wang-q/intspan?category=code)](https://github.com//wang-q/intspan)

## Install

Current release: 0.7.7

```shell
cargo install intspan

cargo install --force --path .

# or
brew install intspan

# local docs
cargo doc --open

# build under WSL 2
mkdir -p /tmp/cargo
export CARGO_TARGET_DIR=/tmp/cargo
cargo build
cargo run --bin fasr help

# build for CentOS 7
# rustup target add x86_64-unknown-linux-gnu
# pip3 install cargo-zigbuild
cargo zigbuild --target x86_64-unknown-linux-gnu.2.17 --release
ll $CARGO_TARGET_DIR/x86_64-unknown-linux-gnu/release/

```

## Concepts

### Ranges

An example is [`S288c.rg`](tests/spanr/S288c.rg).
The information presented in this format is very similar to formats such as the `BED`.

I chose this format because of its compactness, readability, and embeddability into other
tab-separated files.

```text
I:1-100
I(+):90-150
S288c.I(-):190-200
II:21294-22075
II:23537-24097

```

The schema of an `Range` object is shown below.

![ranges](doc/ranges.png)

Simple rules:

* `chromosome` and `start` are required
* `species`, `strand` and `end` are optional
* `.` to separate `species` and `chromosome`
* `strand` is one of `+` and `-` and surround by round brackets
* `:` to separate names and digits
* `-` to separate `start` and `end`
* For `species`:
    * `species` should be alphanumeric with no spaces, the one exception character is `/`.
    * A `species` is an identity that you can also think of as a strain name, an assembly, or
      something else.

```text
species.chromosome(strand):start-end
--------^^^^^^^^^^--------^^^^^^----

```

In this toolset, [`rgr`](#rgr-help) is used to operate ranges in `.rg` and `.tsv` files.

### IntSpans

An IntSpan represents sets of integers as a number of inclusive ranges, for example `1-10,19,45-48`.

The following figure shows the schema of an IntSpan object. Jump lines are above the baseline; loop
lines are below it.

![intspans](doc/intspans.png)

Also, [AlignDB::IntSpan](https://github.com/wang-q/AlignDB-IntSpan) and
[jintspan](https://github.com/egateam/jintspan) are implements of the IntSpan objects in Perl and
Java, respectively.

### Runlists - IntSpans on chromosomes stored in JSON

Very often, we need to deal with many genomic intervals of the same property, e.g., all the exons of
a gene, all the promoters of a gene family, all the repeats in a genome, and so on.

Existing formats, such as `bedGraph`, can partially deal with such situations, but often face
problems of intuitiveness, performance, etc. At the same time, there are only a very limited number
of tools that can handle files in such proprietary formats.

Saving `IntSpan` to a JSON file is the solution of this toolset, where [`spanr`](#spanr-help)
handles this job.

* Single: [`repeat.json`](tests/spanr/repeat.json)

```json
{
    "I": "-",
    "II": "327069-327703",
    "III": "-",
    "IV": "512988-513590,757572-759779,802895-805654,981142-987119,1017673-1018183,1175134-1175738,1307621-1308556,1504223-1504728",
    "IX": "-",
    "V": "354135-354917",
    "VI": "-",
    "VII": "778784-779515,878539-879235",
    "VIII": "116405-117059,133581-134226",
    "X": "366757-367499,712641-713226",
    "XI": "162831-163399",
    "XII": "64067-65208,91960-92481,451418-455181,455933-457732,460517-464318,465070-466869,489753-490545,817840-818474",
    "XIII": "609100-609861",
    "XIV": "-",
    "XV": "437522-438484",
    "XVI": "560481-561065"
}
```

* Multi: [`Atha.json`](tests/spanr/Atha.json)

```json
{
    "AT1G01010.1": {
        "1": "3631-3913,3996-4276,4486-4605,4706-5095,5174-5326,5439-5899"
    },
    "AT1G01020.1": {
        "1": "5928-6263,6437-7069,7157-7232,7384-7450,7564-7649,7762-7835,7942-7987,8236-8325,8417-8464,8571-8737"
    },
    "AT1G01020.2": {
        "1": "6790-7069,7157-7450,7564-7649,7762-7835,7942-7987,8236-8325,8417-8464,8571-8737"
    },
    "AT2G01008.1": {
        "2": "1025-1272,1458-1510,1873-2810,3706-5513,5782-5945"
    },
    "AT2G01021.1": {
        "2": "6571-6672"
    }
}
```

* `chr.sizes`: [`S288c.chr.sizes`](tests/spanr/S288c.chr.sizes)

### Links of ranges

Types of links:

* Bilateral links

        I(+):13063-17220    I(-):215091-219225
        I(+):139501-141431  XII(+):95564-97485

* Bilateral links with hit strand

        I(+):13327-17227    I(+):215084-218967  -
        I(+):139501-141431  XII(+):95564-97485  +

* Multilateral links

        II(+):186984-190356 IX(+):12652-16010   X(+):12635-15993

## Synopsis

### `rgr help`

```text
`rgr` operates ranges in .rg and .tsv files

Usage: rgr [COMMAND]

Commands:
  count    Count each range overlapping with other range files
  field    Create/append ranges from fields
  merge    Merge overlapped ranges via overlapping graph
  prop     Proportion of the ranges intersecting a runlist file
  replace  Replace fields in .tsv file
  runlist  Filter .rg and .tsv files by comparison with a runlist file
  sort     Sort .rg and .tsv files by a range field
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

* Field numbers in the TSV file start at 1

```

### `spanr help`

```text
`spanr` operates chromosome IntSpan files

Usage: spanr [COMMAND]

Commands:
  genome    Convert chr.size to runlists
  some      Extract some records from a runlist json file
  merge     Merge runlist json files
  split     Split a runlist json file
  stat      Coverage on chromosomes for runlists
  statop    Coverage on chromosomes for one JSON crossed another
  combine   Combine multiple sets of runlists in a json file
  compare   Compare one JSON file against others
  span      Operate spans in a JSON file
  cover     Output covers on chromosomes
  coverage  Output minimum or detailed depth of coverage on chromosomes
  gff       Convert gff3 to covers on chromosomes
  convert   Convert runlist file to ranges file
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

```

### `fasr help`

```text
`fasr` operates block fasta files

Usage: fasr [COMMAND]

Commands:
  axt2fas    Convert axt to block fasta
  check      Check genome locations in block fasta headers
  concat     Concatenate sequence pieces of the same species
  consensus  Generate consensus sequences by POA
  cover      Output covers on chromosomes
  create     Create block fasta files from links of ranges
  filter     Filter blocks, and can also be used as a formatter
  join       Join multiple block fasta files by a common target
  link       Output bi/multi-lateral range links
  maf2fas    Convert maf to block fasta
  name       Output all species names
  pl-p2m     Pipeline - pairwise alignments to multiple alignments
  refine     Realign files with external programs and trim unwanted regions
  replace    Concatenate sequence pieces of the same species
  separate   Separate block fasta files by species
  slice      Extract alignment slices
  split      Split block fasta files to per-alignment/chromosome fasta files
  stat       Extract a subset of species
  subset     Extract a subset of species
  variation  List variations (substitutions/indels)
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

```

### `linkr help`

```text
`linkr` operates ranges on chromosomes and links of ranges

Usage: linkr [COMMAND]

Commands:
  circos   Convert links to circos links or highlights
  sort     Sort links and ranges within links
  filter   Filter links by numbers of ranges or length differences
  clean    Replace ranges within links, incorporate hit strands and remove nested links
  connect  Connect bilateral links into multilateral ones
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

```

## Examples

### `spanr`

```shell
spanr genome tests/spanr/S288c.chr.sizes

spanr genome tests/spanr/S288c.chr.sizes |
    spanr stat tests/spanr/S288c.chr.sizes stdin --all

spanr some tests/spanr/Atha.json tests/spanr/Atha.list

spanr merge tests/spanr/I.json tests/spanr/II.json
spanr merge tests/spanr/I.json tests/spanr/II.other.json --all

spanr cover tests/spanr/S288c.rg
spanr cover tests/spanr/dazzname.rg

spanr coverage tests/spanr/S288c.rg -m 2

spanr coverage tests/spanr/S288c.rg -d

spanr gff tests/spanr/NC_007942.gff --tag tRNA

spanr span --op cover tests/spanr/brca2.json

spanr combine tests/spanr/Atha.json

spanr compare \
    --op intersect \
    tests/spanr/intergenic.json \
    tests/spanr/repeat.json

spanr compare \
    --op intersect \
    tests/spanr/I.II.json \
    tests/spanr/I.json \
    tests/spanr/II.json

spanr split tests/spanr/I.II.json

spanr stat tests/spanr/S288c.chr.sizes tests/spanr/intergenic.json

spanr stat tests/spanr/S288c.chr.sizes tests/spanr/I.II.json

spanr stat tests/spanr/Atha.chr.sizes tests/spanr/Atha.json

spanr statop \
    --op intersect \
    tests/spanr/S288c.chr.sizes \
    tests/spanr/intergenic.json \
    tests/spanr/repeat.json

spanr statop \
    --op intersect --all\
    tests/spanr/Atha.chr.sizes \
    tests/spanr/Atha.json \
    tests/spanr/paralog.json

spanr convert tests/spanr/repeat.json tests/spanr/intergenic.json |
    spanr cover stdin |
    spanr stat tests/spanr/S288c.chr.sizes stdin --all

spanr merge tests/spanr/repeat.json tests/spanr/intergenic.json |
    spanr combine stdin |
    spanr stat tests/spanr/S288c.chr.sizes stdin --all

```

### `rgr`

```shell
rgr field tests/Atha/chr.sizes --chr 1 --start 2 -a -s
rgr field tests/spanr/NC_007942.gff -H --chr 1 --start 4 --end 5 --strand 7 --eq 3:tRNA --ne '7:+'
rgr field tests/rgr/ctg.tsv --chr 2 --start 3 --end 4 -H -f 6,1 > tests/rgr/ctg.range.tsv

rgr sort tests/rgr/S288c.rg
rgr sort tests/rgr/ctg.range.tsv -H -f 3
# ctg:I:1 is treated as a range
rgr sort tests/rgr/S288c.rg tests/rgr/ctg.range.tsv

rgr count tests/rgr/S288c.rg tests/rgr/S288c.rg
rgr count tests/rgr/ctg.range.tsv tests/rgr/S288c.rg -H -f 3

rgr runlist tests/rgr/intergenic.json tests/rgr/S288c.rg --op overlap
rgr runlist tests/rgr/intergenic.json tests/rgr/ctg.range.tsv --op non-overlap -H -f 3

rgr prop tests/rgr/intergenic.json tests/rgr/S288c.rg
rgr prop tests/rgr/intergenic.json tests/rgr/ctg.range.tsv -H -f 3 --prefix --full

rgr merge tests/rgr/II.links.tsv -c 0.95

rgr replace tests/rgr/1_4.ovlp.tsv tests/rgr/1_4.replace.tsv
rgr replace tests/rgr/1_4.ovlp.tsv tests/rgr/1_4.replace.tsv -r

# ctg_2_1_.gc.tsv isn't sorted,
cat tests/rgr/ctg_2_1_.gc.tsv | rgr sort stdin | cargo run --bin rgr pl-2rmp stdin > /dev/null
cat tests/rgr/II.links.tsv | cargo run --bin rgr pl-2rmp stdin

cargo run --bin rgr md tests/rgr/ctg.range.tsv --num -c 2

```

### `linkr`

```shell
linkr sort tests/linkr/II.links.tsv -o tests/linkr/II.sort.tsv

rgr merge tests/linkr/II.links.tsv -v

linkr clean tests/linkr/II.sort.tsv
linkr clean tests/linkr/II.sort.tsv --bundle 500
linkr clean tests/linkr/II.sort.tsv -r tests/linkr/II.merge.tsv

linkr connect tests/linkr/II.clean.tsv -v

linkr filter tests/linkr/II.connect.tsv -n 2
linkr filter tests/linkr/II.connect.tsv -n 3 -r 0.99

linkr circos tests/linkr/II.connect.tsv
linkr circos --highlight tests/linkr/II.connect.tsv

```

Steps:

```text
    sort
      |
      v
    clean -> merge
      |     /
      |  /
      v
    clean
      |
      V
    connect
      |
      v
    filter
```

#### S288c

```shell
linkr sort tests/S288c/links.lastz.tsv tests/S288c/links.blast.tsv \
    -o tests/S288c/sort.tsv

linkr clean tests/S288c/sort.tsv \
    -o tests/S288c/sort.clean.tsv

rgr merge tests/S288c/sort.clean.tsv -c 0.95 \
    -o tests/S288c/merge.tsv

linkr clean tests/S288c/sort.clean.tsv -r tests/S288c/merge.tsv --bundle 500 \
    -o tests/S288c/clean.tsv

linkr connect tests/S288c/clean.tsv -r 0.8 \
    -o tests/S288c/connect.tsv

linkr filter tests/S288c/connect.tsv -r 0.8 \
    -o tests/S288c/filter.tsv

wc -l tests/S288c/*.tsv
#     229 tests/S288c/clean.tsv
#     148 tests/S288c/connect.tsv
#     148 tests/S288c/filter.tsv
#     566 tests/S288c/links.blast.tsv
#     346 tests/S288c/links.lastz.tsv
#      74 tests/S288c/merge.tsv
#     282 tests/S288c/sort.clean.tsv
#     626 tests/S288c/sort.tsv

cat tests/S288c/filter.tsv |
    perl -nla -F"\t" -e 'print for @F' |
    spanr cover stdin -o tests/S288c/cover.json

spanr stat tests/S288c/chr.sizes tests/S288c/cover.json -o stdout

```

#### Atha

```shell
gzip -dcf tests/Atha/links.lastz.tsv.gz tests/Atha/links.blast.tsv.gz |
    linkr sort stdin -o tests/Atha/sort.tsv

linkr clean tests/Atha/sort.tsv -o tests/Atha/sort.clean.tsv

rgr merge tests/Atha/sort.clean.tsv -c 0.95 -o tests/Atha/merge.tsv

linkr clean tests/Atha/sort.clean.tsv -r tests/Atha/merge.tsv --bundle 500 -o tests/Atha/clean.tsv

linkr connect tests/Atha/clean.tsv -o tests/Atha/connect.tsv

linkr filter tests/Atha/connect.tsv -r 0.8 -o tests/Atha/filter.tsv

wc -l tests/Atha/*.tsv
#    4500 tests/Atha/clean.tsv
#    3832 tests/Atha/connect.tsv
#    3832 tests/Atha/filter.tsv
#     785 tests/Atha/merge.tsv
#    5416 tests/Atha/sort.clean.tsv
#    7754 tests/Atha/sort.tsv

cat tests/Atha/filter.tsv |
    perl -nla -F"\t" -e 'print for @F' |
    spanr cover stdin -o tests/Atha/cover.json

spanr stat tests/Atha/chr.sizes tests/Atha/cover.json -o stdout

```

### `fasr`

```shell
fasr maf2fas tests/fasr/example.maf

fasr axt2fas tests/fasr/RM11_1a.chr.sizes tests/fasr/example.axt --qname RM11_1a

cargo run --bin fasr filter tests/fasr/example.fas --ge 10

fasr name tests/fasr/example.fas --count

fasr cover tests/fasr/example.fas

fasr cover tests/fasr/example.fas --name S288c --trim 10

fasr concat tests/fasr/name.lst tests/fasr/example.fas

fasr subset tests/fasr/name.lst tests/fasr/example.fas
cargo run --bin fasr subset tests/fasr/name.lst tests/fasr/refine.fas --required

fasr link tests/fasr/example.fas --pair
fasr link tests/fasr/example.fas --best

cargo run --bin fasr replace tests/fasr/replace.tsv tests/fasr/example.fas
cargo run --bin fasr replace tests/fasr/replace.fail.tsv tests/fasr/example.fas

samtools faidx tests/fasr/NC_000932.fa NC_000932:1-10

fasr check tests/fasr/NC_000932.fa tests/fasr/A_tha.pair.fas

fasr create tests/fasr/genome.fa tests/fasr/I.connect.tsv --name S288c

# Create a fasta file containing multiple genomes
cat tests/fasr/genome.fa | sed 's/^>/>S288c./' > tests/fasr/genomes.fa
samtools faidx tests/fasr/genomes.fa S288c.I:1-100

cargo run --bin fasr create tests/fasr/genomes.fa tests/fasr/I.name.tsv --multi

fasr separate tests/fasr/example.fas -o . --suffix .tmp

spoa tests/fasr/refine.fasta -r 1

cargo run --bin fasr consensus tests/fasr/example.fas
cargo run --bin fasr consensus tests/fasr/refine.fas
cargo run --bin fasr consensus tests/fasr/refine.fas --outgroup -p 2

cargo run --bin fasr refine tests/fasr/example.fas
cargo run --bin fasr refine tests/fasr/example.fas --msa none --chop 10
cargo run --bin fasr refine tests/fasr/refine2.fas --msa clustalw --outgroup
cargo run --bin fasr refine tests/fasr/example.fas --quick

cargo run --bin fasr split tests/fasr/example.fas --simple
cargo run --bin fasr split tests/fasr/example.fas -o . --chr --suffix .tmp

cargo run --bin fasr slice tests/fasr/slice.json tests/fasr/slice.fas --name S288c

cargo run --bin fasr join tests/fasr/S288cvsYJM789.slice.fas --name YJM789
cargo run --bin fasr join \
    tests/fasr/S288cvsRM11_1a.slice.fas \
    tests/fasr/S288cvsYJM789.slice.fas \
    tests/fasr/S288cvsSpar.slice.fas

cargo run --bin fasr stat tests/fasr/example.fas --outgroup

cargo run --bin fasr variation tests/fasr/example.fas
cargo run --bin fasr variation tests/fasr/example.fas --outgroup

cargo run --bin fasr xlsx tests/fasr/example.fas
cargo run --bin fasr xlsx tests/fasr/example.fas --outgroup

cargo run --bin fasr pl-p2m tests/fasr/S288cvsRM11_1a.slice.fas tests/fasr/S288cvsSpar.slice.fas

```

## License

[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2Fwang-q%2Fintspan.svg?type=large)](https://app.fossa.io/projects/git%2Bgithub.com%2Fwang-q%2Fintspan?ref=badge_large)
