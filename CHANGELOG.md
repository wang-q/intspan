# Change Log

## Unreleased - ReleaseDate

## 0.8.7 - 2025-04-06

* Add linear algebra functions
* Add matrix operations
    * Add `ScoringMatrix` for pairwise scores with missing values
    * Add `NamedMatrix` for complete distance matrices in PHYLIP format
    * Support PHYLIP format input
* Remove AppVeyor CI

## 0.8.6 - 2025-04-02

* Add `--ff-eq` and `--ff-ne` to `rgr filter`
* Improve code quality
    * Use `into_iter()` and `collect()` to simplify code
    * Refactor `IntSpan::to_vec()`, `spans()`, `ranges()`, `runs()` and `intses()`
* Improve CI/CD
    * Use `cargo-zigbuild` to build linux-gnu binary with GLIBC 2.17
    * Simplify tar archives by removing path prefixes

## 0.8.4 - 2024-12-30

* Add `--lines` and `--delete` to `rgr keep`

* Refactor the code in `rgr` to make it cleaner and more maintainable

## 0.8.3 - 2024-12-29

* Add `rgr span`
* Add `rgr keep`

* Add numeric comparisons to `rgr filter`
* Add operations to `Range`
* Remove --fields from `rgr field`

## 0.8.2 - 2024-12-21

* Add `IntSpan::valid()`

* Add `rgr filter`
* Add `rgr select`

## 0.8.0 - 2024-11-30

* Move `fasr` to `hnsm`

* Add `--longest` to `spanr convert`
* Format Markdown tables in `rgr md`

## 0.7.9 - 2024-11-15

* Add `rgr dedup`

* Add --fmt to `rgr md`

## 0.7.8 - 2024-11-04

* Add `rgr md`

## 0.7.7 - 2024-07-19

* Move `ovlpr` to `anchr`

* Add `rgr pl-2rmp`
* Add `--group` to `rgr sort`
* Use `MultiGzDecoder` in intspan::reader()

## 0.7.3 - 2023-07-18

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

## 0.4.16 - 2022-02-12

* Switch to `clap` v3

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
