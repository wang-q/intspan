# Change Log

## [Unreleased] - ReleaseDate

### Changed

* `spanr merge`: take the first part of filename
* `spanr compare`: more than two infiles

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
