# Change Log

## Unreleased - ReleaseDate

## 0.7.7 - 2024-07-19

* Move `ovlpr` to `anchr`
* Add `rgr pl-2rmp`
* Add `--group` to `rgr sort`
* Use `MultiGzDecoder` in intspan::reader()

## 0.7.5 - 2023-07-28

* Add `fasr replace`
* Add `fasr pl-p2m`
* Add `fasr xlsx`

* Add --outgroup to `fasr consensus` and `fasr variation`
* Add --required to `fasr subset`
* Add --outgroup, --chop, and --quick to `fasr refine`

* Implementing `--parallel` with `crossbeam`
    * for `fasr consensus`
    * for `fasr refine`

## 0.7.4 - 2023-07-20

* Add `fasr filter`
* Add `fasr refine`
* Add `fasr variation`

* Add --multi to `fasr create`

## 0.7.3 - 2023-07-18

* New binary `fasr`, ported from `App::Fasops`
    * `fasr axt2fas`
    * `fasr check`
    * `fasr concat`
    * `fasr consensus`
    * `fasr cover`
    * `fasr create`
    * `fasr join`
    * `fasr link`
    * `fasr maf2fas`
    * `fasr name`
    * `fasr separate`
    * `fasr slice`
    * `fasr split`
    * `fasr stat`
    * `fasr subset`

* Add `libs/alignment`

* Add `utils::get_seq_faidx()`
* Add `IntSpan.find_islands_n()` and `IntSpan.find_islands_ints()`

* Use json to replace yaml in `spanr`

* Bump versions of deps
    * clap v4
    * Use anyhow

## 0.7.1 - 2022-06-14

* Store `IntSpan.edges` in VecDeque
* Switch to `clap` v3.2

## 0.7.0 - 2022-05-23

* Add `rgr sort`
* Add `rgr prop`

* Add --fields to `rgr field`
* Add --header, --sharp, --field to `rgr count` and `rgr runlist`

## 0.6.9 - 2022-05-15

* Add `rgr field`

* Move `spanr range` to `rgr runlist`
* Move `spanr count` to `rgr count`

* Rename .ranges to .rg

## 0.6.8 - 2022-05-14

* New binary `rgr`

* Move `ovlpr replace` to `rgr replace`
* Move `linkr merge` to `rgr merge`

## 0.6.7 - 2022-04-24

* Add `spanr count`
* Add `--detailed` to `spanr coverage`
* Use `Box<dyn std::error::Error>`

## 0.6.5 - 2022-04-22

* Use rust_lapper as an intermediate layer instead of intspan::Coverage
    * Greatly improves the speed of `spanr coverage`

## 0.6.4 - 2022-04-21

* Move `far` out
* `spanr stat` use i64 in the `all` lines
* Update Github actions
    * Use a container with GLIBC 2.17 to build linux-gnu binary
    * Codecov with cargo-tarpaulin

## 0.6.0 - 2022-02-22

* Move `nwr` out

## 0.4.17 - 2022-02-21

* Add `nwr download`
* Add `nwr txdb`
* Add `nwr info`
* Add `nwr lineage`
* Add `nwr restrict`
* Add `nwr member`
* Add `nwr append`

## 0.4.16 - 2022-02-12

* Switch to `clap` v3
* New binary `nwr`

## 0.4.15 - 2021-08-19

* Add `far some`
* Add `slice()` to `IntSpan`
* `ovlpr replace` now processes any .tsv files

## 0.4.14 - 2020-05-15

## 0.4.13 - 2020-05-15

* New binary `far`

## 0.4.12 - 2020-03-05

* Split `spanr cover` into `cover` and `coverage`

## 0.4.11 - 2020-02-15

* Add `--all` to `spanr merge`

## 0.4.10 - 2020-02-15

* Add `--op` to `spanr combine`

## 0.4.9 - 2019-12-09

* Add `ovlpr replace`
* Add `ovlpr restrict`

* Github Actions publish.yml

## 0.4.1 - 2019-09-10

* Add benchmarks.md
* Add `ovlpr paf2ovlp`

* Binary releases by Github Actions

## 0.4.0 - 2019-09-07

* New binary `ovlpr`
* Struct `Overlap`

* Move libraries to libs/
* Passing `&str` when calling methods
* Add `new_len()` and `uniq_tiers()` to `Coverage`
* Add `from_pair()` to `IntSpan`
* Wrap IO functions in utils.rs with Result
* Satisfy clippy

## 0.3.3 - 2019-09-04

* `spanr merge`: take the first part of filename
* `spanr compare`: compare more than two infiles

## 0.3.2 - 2019-09-03

* Add `--suffix` to `spanr split`

## 0.3.1 - 2019-09-03

* Detailed benchmarks on `linkr`

* Make POS_INF, NEG_INF and EMPTY_STRING as lazy_static
* About 10-20% faster

## 0.3.0 - 2019-09-03

* New binary `linkr` for commands ported from `App::Rangeops` and `jrange`
* Illustrations for some concepts
    * IntSpans
    * Ranges

* Rename binary `intspan` to `spanr`

## 0.2.0 - 2019-08-24

* Ported all commands from `App::RL` and `jrunlist`
* Struct `Range`
* Struct `Coverage`
* Adopt `cargo release`

## 0.1.0 - 2019-08-13

* Struct `IntSpan`
* Examples
