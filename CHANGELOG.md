# Change Log

Unreleased ReleaseDate

## [0.4.2] - 2019-12-09

### Added

* Add `ovlpr replace`
* Add `ovlpr restrict`

### Changed

* Github Actions publish.yml

## [0.4.1] - 2019-09-10

### Added

* Add benchmarks.md
* Add `ovlpr paf2ovlp`

### Changed

* Binary releases by Github Actions

## [0.4.0] - 2019-09-07

### Added

* New binary `ovlpr` 
* Struct `Overlap`

### Changed

* Move libraries to libs/
* Passing `&str` when calling methods
* Add `new_len()` and `uniq_tiers()` to `Coverage`
* Add `from_pair()` to `IntSpan`
* Wrap IO functions in utils.rs with Result
* Satisfy clippy

## [0.3.3] - 2019-09-04

### Changed

* `spanr merge`: take the first part of filename
* `spanr compare`: compare more than two infiles

## [0.3.2] - 2019-09-03

### Added

* Add `--suffix` to `spanr split`

## [0.3.1] - 2019-09-03

### Added

* Detailed benchmarks on `linkr`

### Changed

* Make POS_INF, NEG_INF and EMPTY_STRING as lazy_static
* About 10-20% faster

## [0.3.0] - 2019-09-03

### Added

* New binary `linkr` for commands ported from `App::Rangeops` and `jrange`
* Illustrations for some concepts
    * IntSpans
    * Ranges

### Changed

* Rename binary `intspan` to `spanr`

## [0.2.0] - 2019-08-24

### Added

* Ported all commands from `App::RL` and `jrunlist`
* Struct `Range`
* Struct `Coverage`
* Adopt `cargo release`

## [0.1.0] - 2019-08-13

### Added

* Struct `IntSpan`
* Examples
