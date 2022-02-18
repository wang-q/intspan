# Change Log

## Unreleased - ReleaseDate

* Add `nwr download`
* Add `nwr txdb`

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

## 0.4.12 -  2020-03-05

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
