# intspan

[![Linux build status](https://travis-ci.org/wang-q/intspan.svg)](https://travis-ci.org/wang-q/intspan)
[![Windows build status](https://ci.appveyor.com/api/projects/status/github/wang-q/intspan?svg=true)](https://ci.appveyor.com/project/wang-q/intspan)
[![Codecov branch](https://img.shields.io/codecov/c/github/wang-q/intspan/master.svg)](https://codecov.io/github/wang-q/intspan?branch=master)
[![Crates.io](https://img.shields.io/crates/v/intspan.svg)](https://crates.io/crates/intspan)
[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2Fwang-q%2Fintspan.svg?type=shield)](https://app.fossa.io/projects/git%2Bgithub.com%2Fwang-q%2Fintspan?ref=badge_shield)

## Install

Current release: 0.2.0

`cargo install intspan`

## Concepts

### IntSpans

An IntSpan represents sets of integers as a number of inclusive ranges, for example `1-10,19,45-48`
or `-99--10,1-10,19,45-48`.

The following picture is the schema of an IntSpan object. Jump lines are above the baseline; loop
lines are below it.

![intspans](doc/intspans.png)

Also, [AlignDB::IntSpan](https://github.com/wang-q/AlignDB-IntSpan) and
[jintspan](https://github.com/egateam/jintspan) are implements of IntSpan objects in Perl and Java,
respectively.

### IntSpans on chromosomes

* `chr.sizes`

* Single

* Multi

### Ranges

Examples in [`S288c.ranges`](tests/resources/S288c.ranges)

```text
I:1-100
I(+):90-150
S288c.I(-):190-200
II:21294-22075
II:23537-24097
```

![ranges](doc/ranges.png)

Simple rules:

* `chromosome` and `start` are required
* `species`, `strand` and `end` are optional
* `.` to separate `species` and `chromosome`
* `strand` is one of `+` and `-` and surround by round brackets
* `:` to separate names and digits
* `-` to separate `start` and `end`
* For `species`:
    * `species` should be alphanumeric and without spaces, one exception char is `/`.
    * `species` is an identity, you can also treat is as a strain name, a lineage or something else. 

```text
species.chromosome(strand):start-end
--------^^^^^^^^^^--------^^^^^^----
```

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

## EXAMPLES

### `intspan`

```bash
# cargo install --path . --force

intspan genome tests/resources/S288c.chr.sizes

intspan genome tests/resources/S288c.chr.sizes |
    intspan stat tests/resources/S288c.chr.sizes stdin --all

intspan some tests/resources/Atha.yml tests/resources/Atha.list

intspan merge tests/resources/I.yml tests/resources/II.yml

intspan cover tests/resources/S288c.ranges
intspan cover tests/resources/S288c.ranges -c 2
intspan cover tests/resources/dazzname.ranges

intspan gff tests/resources/NC_007942.gff --tag tRNA

intspan range --op overlap tests/resources/intergenic.yml tests/resources/S288c.ranges

intspan span --op cover tests/resources/brca2.yml

intspan combine tests/resources/Atha.yml
jrunlist combine -o stdout tests/resources/Atha.yml

intspan compare \
    --op intersect \
    tests/resources/intergenic.yml \
    tests/resources/repeat.yml

intspan split tests/resources/I.II.yml

intspan stat tests/resources/S288c.chr.sizes tests/resources/intergenic.yml

intspan stat tests/resources/S288c.chr.sizes tests/resources/I.II.yml

diff <(intspan stat tests/resources/Atha.chr.sizes tests/resources/Atha.yml) \
    <(jrunlist stat -o stdout tests/resources/Atha.chr.sizes tests/resources/Atha.yml)

intspan statop \
    --op intersect \
    tests/resources/S288c.chr.sizes \
    tests/resources/intergenic.yml \
    tests/resources/repeat.yml

diff <(intspan statop \
        --op intersect --all\
        tests/resources/Atha.chr.sizes \
        tests/resources/Atha.yml \
        tests/resources/paralog.yml ) \
    <(jrunlist statop \
        -o stdout \
        --op intersect --all \
        tests/resources/Atha.chr.sizes \
        tests/resources/Atha.yml \
        tests/resources/paralog.yml )

intspan convert tests/resources/repeat.yml tests/resources/intergenic.yml |
    intspan cover stdin |
    intspan stat tests/resources/S288c.chr.sizes stdin --all
intspan merge tests/resources/repeat.yml tests/resources/intergenic.yml |
    intspan combine stdin |
    intspan stat tests/resources/S288c.chr.sizes stdin --all

```

### `linkr`

```bash
# cargo install --path . --force

target/debug/linkr circos tests/linkr/II.connect.tsv
target/debug/linkr circos --highlight tests/linkr/II.connect.tsv

diff <(target/debug/linkr sort tests/linkr/II.links.tsv) \
    tests/linkr/II.sort.tsv

target/debug/linkr merge -v tests/linkr/II.links.tsv

target/debug/linkr filter tests/linkr/II.connect.tsv -n 2
target/debug/linkr filter tests/linkr/II.connect.tsv -n 3 -r 0.99

target/debug/linkr clean tests/linkr/II.sort.tsv
target/debug/linkr clean tests/linkr/II.sort.tsv --bundle 500 
target/debug/linkr clean tests/linkr/II.sort.tsv -r tests/linkr/II.merge.tsv

```

## Benchmark 1

* Rust

```text
$ cd ~/Scripts/rust/intspan
$ cargo build --release --examples
$ command time -l target/release/examples/benchmark
["target/release/examples/benchmark"]
step 2
duration: 0.04019228699999999 s
step 3
duration: 0.048044463 s
step 4
duration: 0.101135046 s
step 5
duration: 0.378076464 s
step 6
duration: 0.6468764170000001 s
        1.21 real         1.21 user         0.00 sys
   1024000  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
       259  page reclaims
         0  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         0  signals received
         0  voluntary context switches
       304  involuntary context switches

```

* Java

```text
$ cd ~/Scripts/java/jintspan
$ mvn clean verify
$ command time -l java -jar target/jintspan-*-jar-with-dependencies.jar benchmark
step 2
duration 0.023358
step 3
duration 0.035295
step 4
duration 0.053588
step 5
duration 0.316216
step 6
duration 0.561500
        1.12 real         1.38 user         0.07 sys
 110718976  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
     30334  page reclaims
         4  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         1  signals received
         4  voluntary context switches
       926  involuntary context switches

```

* C

```text
$ cd ~/Scripts/cpan/AlignDB-IntSpanXS/benchmark
$ make
$ command time -l ./test_c benchmark
step 2
duration 0.022875
step 3
duration 0.032172
step 4
duration 0.057164
step 5
duration 0.294729
step 6
duration 0.525069
        0.93 real         0.93 user         0.00 sys
   1085440  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
       274  page reclaims
         0  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         0  signals received
         0  voluntary context switches
       176  involuntary context switches

```

* Perl

```text
$ cd ~/Scripts/cpan/AlignDB-IntSpanXS/benchmark
$ command time -l perl test_ai.pl benchmark
step 2
duration 2.506869
step 3
duration 2.831008
step 4
duration 2.969270
step 5
duration 46.395918
step 6
duration 96.724945
      151.45 real       151.25 user         0.10 sys
   6377472  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
      1566  page reclaims
         0  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         0  signals received
         0  voluntary context switches
     14911  involuntary context switches

```

* Perl XS

```text
$ cd ~/Scripts/cpan/AlignDB-IntSpanXS/benchmark
$ command time -l perl test_ai.pl benchmark xs
step 2
duration 0.273726
step 3
duration 0.296036
step 4
duration 0.344481
step 5
duration 2.072225
step 6
duration 9.789098
       12.80 real        12.76 user         0.02 sys
   6475776  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
      1590  page reclaims
         0  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         0  signals received
         0  voluntary context switches
      3016  involuntary context switches

```

## Benchmark 2

* Rust

```text
$ cd ~/Scripts/rust/intspan
$ cargo build --release --examples
$ command time -l target/release/examples/file
["target/release/examples/file"]
step 1 create
duration: 0.022158192 s
step 2 intersect
duration: 0.846951539 s
step 3 intersect runlist
duration: 0.9379064089999999 s
        1.81 real         1.80 user         0.00 sys
   2555904  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
       633  page reclaims
         0  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         0  signals received
         0  voluntary context switches
       383  involuntary context switches

```

* Java

```text
$ cd ~/Scripts/java/jintspan
$ mvn clean verify
$ command time -l java -jar target/jintspan-*-jar-with-dependencies.jar file
step 1 create
duration 0.071450
step 2 intersect
duration 0.499175
step 3 intersect runlist
duration 0.789997
        1.52 real         1.69 user         0.14 sys
 308686848  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
     78554  page reclaims
         2  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         1  signals received
         3  voluntary context switches
      2034  involuntary context switches

```

* C

```text
$ cd ~/Scripts/cpan/AlignDB-IntSpanXS/benchmark
$ make
$ command time -l ./test_c file
step 1 create
duration 0.118375
step 2 intersect
duration 2.174462
step 3 intersect runlist
duration 18.218233
       20.51 real        20.42 user         0.05 sys
   2121728  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
       521  page reclaims
         6  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         0  signals received
         1  voluntary context switches
      6360  involuntary context switches

```

* Perl

```text
$ cd ~/Scripts/cpan/AlignDB-IntSpanXS/benchmark
$ command time -l perl test_ai.pl file
==> test against large sets
step 1 create
duration 4.548069
step 2 intersect
duration 61.313397
step 3 intersect runlist
duration 61.335031
      127.25 real       126.56 user         0.38 sys
  11943936  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
      2924  page reclaims
         1  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         0  signals received
         6  voluntary context switches
     45869  involuntary context switches

```

* Perl XS

```text
$ cd ~/Scripts/cpan/AlignDB-IntSpanXS/benchmark
$ command time -l perl test_ai.pl file xs
==> test against large sets
step 1 create
duration 0.116019
step 2 intersect
duration 8.530752
step 3 intersect runlist
duration 8.677303
       17.37 real        17.26 user         0.05 sys
   9822208  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
      2407  page reclaims
         0  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         0  signals received
         0  voluntary context switches
      7011  involuntary context switches

```


## License
[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2Fwang-q%2Fintspan.svg?type=large)](https://app.fossa.io/projects/git%2Bgithub.com%2Fwang-q%2Fintspan?ref=badge_large)