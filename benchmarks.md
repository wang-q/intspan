# Benchmarks of different implementations

## IntSpan 1

* Rust

```text
$ pushd ~/Scripts/rust/intspan && 
    cargo build --release --examples && 
    command time -l target/release/examples/benchmark && 
    popd
["target/release/examples/benchmark"]
step 2
duration: 0.024120771000000003 s
step 3
duration: 0.028843299 s
step 4
duration: 0.067436723 s
step 5
duration: 0.26832116899999997 s
step 6
duration: 0.470739197 s
        0.86 real         0.85 user         0.00 sys
   1073152  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
       271  page reclaims
         0  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         0  signals received
         0  voluntary context switches
        42  involuntary context switches

```

* Java

```text
$ pushd ~/Scripts/java/jintspan && 
    mvn clean verify && 
    command time -l java -jar target/jintspan-*-jar-with-dependencies.jar benchmark && 
    popd
step 2
duration 0.015188
step 3
duration 0.023388
step 4
duration 0.042441
step 5
duration 0.212403
step 6
duration 0.365264
        0.78 real         0.93 user         0.05 sys
 182984704  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
     46445  page reclaims
         1  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         1  signals received
         5  voluntary context switches
       654  involuntary context switches

```

* C

```text
$ pushd ~/Scripts/cpan/AlignDB-IntSpanXS/benchmark &&
    make &&
    command time -l ./test_c benchmark &&
    popd
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
$ pushd ~/Scripts/cpan/AlignDB-IntSpanXS/benchmark &&
    command time -l perl test_ai.pl benchmark &&
    popd
step 2
duration 1.749132
step 3
duration 1.951063
step 4
duration 2.060910
step 5
duration 32.967968
step 6
duration 68.138299
      106.89 real       106.77 user         0.05 sys
   6500352  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
      1596  page reclaims
         0  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         0  signals received
         0  voluntary context switches
      6303  involuntary context switches

```

* Perl XS

```text
$ pushd ~/Scripts/cpan/AlignDB-IntSpanXS/benchmark &&
    command time -l perl test_ai.pl benchmark xs &&
    popd
step 2
step 2
duration 0.195549
step 3
duration 0.205843
step 4
duration 0.271525
step 5
duration 1.440861
step 6
duration 6.663916
        8.79 real         8.78 user         0.00 sys
   6586368  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
      1617  page reclaims
         0  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         0  signals received
         0  voluntary context switches
       609  involuntary context switches

```

## IntSpan 2

* Rust

```text
$ pushd ~/Scripts/rust/intspan && 
    cargo build --release --examples && 
    command time -l target/release/examples/file && 
    popd
["target/release/examples/file"]
step 1 create
duration: 0.014380602999999999 s
step 2 intersect
duration: 0.523088785 s
step 3 intersect runlist
duration: 0.611574268 s
        1.15 real         1.14 user         0.00 sys
   2699264  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
       668  page reclaims
         0  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         0  signals received
         0  voluntary context switches
       243  involuntary context switches

```

* Java

```text
$ pushd ~/Scripts/java/jintspan && 
    mvn clean verify && 
    command time -l java -jar target/jintspan-*-jar-with-dependencies.jar file && 
    popd
step 1 create
duration 0.053408
step 2 intersect
duration 0.347398
step 3 intersect runlist
duration 0.555934
        1.06 real         1.17 user         0.12 sys
 582258688  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
    143954  page reclaims
         0  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         1  signals received
         0  voluntary context switches
      1194  involuntary context switches

```

* C

```text
$ pushd ~/Scripts/cpan/AlignDB-IntSpanXS/benchmark &&
    make &&
    command time -l ./test_c file &&
    popd
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
$ pushd ~/Scripts/cpan/AlignDB-IntSpanXS/benchmark &&
    command time -l perl test_ai.pl file &&
    popd
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
$ pushd ~/Scripts/cpan/AlignDB-IntSpanXS/benchmark &&
    command time -l perl test_ai.pl file xs &&
    popd
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

## `linkr` on macOS

* macOS 10.14
* i7-8700k 
* oracleJDK8
* rustc 1.37.0
* Perl 5.30.0

```bash
brew install jrange
cargo install intspan
cpanm App::Rangeops

cargo install hyperfine

```

### sort

```bash
hyperfine --warmup 1 --export-markdown sort.md.tmp \
    'gzip -dcf tests/Atha/links.lastz.tsv.gz tests/Atha/links.blast.tsv.gz | linkr    sort stdin -o /dev/null' \
    'gzip -dcf tests/Atha/links.lastz.tsv.gz tests/Atha/links.blast.tsv.gz | jrange   sort stdin -o /dev/null' \
    'gzip -dcf tests/Atha/links.lastz.tsv.gz tests/Atha/links.blast.tsv.gz | rangeops sort stdin -o /dev/null'

```

| Command  |     Mean [ms] | Min [ms] | Max [ms] | Relative |
| :------- | ------------: | -------: | -------: | -------: |
| linkr    |   143.4 ± 1.4 |    141.5 |    147.2 |      1.0 |
| jrange   |   604.3 ± 4.0 |    599.9 |    613.2 |      4.2 |
| rangeops | 1399.3 ± 14.6 |   1380.5 |   1423.1 |      9.8 |

### clean

```bash
hyperfine --min-runs 3 --export-markdown clean.md.tmp \
    'linkr    clean tests/Atha/sort.tsv -o /dev/null' \
    'jrange   clean tests/Atha/sort.tsv -o /dev/null' \
    'rangeops clean tests/Atha/sort.tsv -o /dev/null'

```

| Command  |        Mean [s] | Min [s] | Max [s] | Relative |
| :------- | --------------: | ------: | ------: | -------: |
| linkr    |   5.896 ± 0.045 |   5.864 |   5.948 |      1.2 |
| jrange   |   4.881 ± 0.049 |   4.831 |   4.929 |      1.0 |
| rangeops | 122.840 ± 1.813 | 120.871 | 124.442 |     25.2 |

### merge

```bash
hyperfine --min-runs 3 --export-markdown merge.md.tmp \
    'linkr    merge tests/Atha/sort.clean.tsv -c 0.95 -o /dev/null' \
    'jrange   merge tests/Atha/sort.clean.tsv -c 0.95 -o /dev/null' \
    'rangeops merge tests/Atha/sort.clean.tsv -c 0.95 -p 4 -o /dev/null'

```

| Command  |       Mean [s] | Min [s] | Max [s] | Relative |
| :------- | -------------: | ------: | ------: | -------: |
| linkr    |  5.278 ± 0.026 |   5.255 |   5.305 |      2.4 |
| jrange   |  2.228 ± 0.020 |   2.206 |   2.246 |      1.0 |
| rangeops | 64.090 ± 0.267 |  63.783 |  64.273 |     28.8 |

### clean2

```bash
hyperfine --min-runs 3 --export-markdown clean2.md.tmp \
    'linkr    clean tests/Atha/sort.clean.tsv -r tests/Atha/merge.tsv --bundle 500 -o /dev/null' \
    'jrange   clean tests/Atha/sort.clean.tsv -r tests/Atha/merge.tsv --bundle 500 -o /dev/null' \
    'rangeops clean tests/Atha/sort.clean.tsv -r tests/Atha/merge.tsv --bundle 500 -o /dev/null'

```

| Command  |       Mean [s] | Min [s] | Max [s] | Relative |
| :------- | -------------: | ------: | ------: | -------: |
| linkr    |  3.208 ± 0.019 |   3.186 |   3.220 |      1.0 |
| jrange   |  4.170 ± 0.047 |   4.119 |   4.212 |      1.3 |
| rangeops | 54.888 ± 0.244 |  54.651 |  55.139 |     17.1 |

### connect

```bash
hyperfine --export-markdown connect.md.tmp \
    'linkr    connect tests/Atha/clean.tsv -o /dev/null' \
    'rangeops connect tests/Atha/clean.tsv -o /dev/null'

```

| Command  |     Mean [ms] | Min [ms] | Max [ms] | Relative |
| :------- | ------------: | -------: | -------: | -------: |
| linkr    |   196.9 ± 1.9 |    194.5 |    200.6 |      1.0 |
| rangeops | 2497.3 ± 11.5 |   2477.2 |   2513.0 |     12.7 |

### filter

```bash
hyperfine --warmup 1 --export-markdown filter.md.tmp \
    'linkr    filter tests/Atha/connect.tsv -r 0.8 -o /dev/null' \
    'jrange   filter tests/Atha/connect.tsv -r 0.8 -o /dev/null' \
    'rangeops filter tests/Atha/connect.tsv -r 0.8 -o /dev/null'

```

| Command  |   Mean [ms] | Min [ms] | Max [ms] | Relative |
| :------- | ----------: | -------: | -------: | -------: |
| linkr    |  31.0 ± 0.6 |     30.1 |     33.0 |      1.0 |
| jrange   | 109.6 ± 1.9 |    105.4 |    114.2 |      3.5 |
| rangeops | 415.7 ± 6.4 |    409.3 |    429.2 |     13.4 |

## `linkr` on Windows

* E3-1245 V2
* Windows 10 18950
* strawberry-perl-5.30.0.1-64bit
* rustc 1.39.0-nightly (c6e9c76c5 2019-09-04) msvc

### sort

```cmd
hyperfine --warmup 1 --export-markdown sort.md.tmp "gzip -dcf tests/Atha/links.lastz.tsv.gz tests/Atha/links.blast.tsv.gz | linkr sort stdin > NUL" "gzip -dcf tests/Atha/links.lastz.tsv.gz tests/Atha/links.blast.tsv.gz | rangeops sort stdin -o stdout > NUL"

```

| Command  |     Mean [ms] | Min [ms] | Max [ms] | Relative |
| :------- | ------------: | -------: | -------: | -------: |
| linkr    |  298.2 ± 12.6 |    279.3 |    319.7 |      1.0 |
| rangeops | 3818.2 ± 85.7 |   3642.8 |   3918.2 |     12.8 |

### clean

```cmd
hyperfine --min-runs 3 --export-markdown clean.md.tmp "linkr clean tests/Atha/sort.tsv > NUL" "rangeops clean tests/Atha/sort.tsv -o stdout > NUL"

```

| Command  |        Mean [s] | Min [s] | Max [s] | Relative |
| :------- | --------------: | ------: | ------: | -------: |
| linkr    |  10.254 ± 0.240 |  10.061 |  10.522 |      1.0 |
| rangeops | 238.339 ± 4.120 | 235.504 | 243.065 |     23.2 |

### merge

```cmd
hyperfine --min-runs 3 --export-markdown merge.md.tmp "linkr merge tests/Atha/sort.clean.tsv -c 0.95 > NUL" "rangeops merge tests/Atha/sort.clean.tsv -c 0.95 -p 4 -o stdout > NUL"

```

| Command  |         Mean [s] | Min [s] | Max [s] | Relative |
| :------- | ---------------: | ------: | ------: | -------: |
| linkr    |    9.749 ± 0.379 |   9.499 |  10.185 |      1.0 |
| rangeops | 166.274 ± 10.915 | 153.870 | 174.408 |     17.1 |

### clean2

```cmd
hyperfine --min-runs 3 --export-markdown clean2.md.tmp "linkr clean tests/Atha/sort.clean.tsv -r tests/Atha/merge.tsv --bundle 500 > NUL" "rangeops clean tests/Atha/sort.clean.tsv -r tests/Atha/merge.tsv --bundle 500 > NUL"

```

| Command  |        Mean [s] | Min [s] | Max [s] | Relative |
| :------- | --------------: | ------: | ------: | -------: |
| linkr    |   5.101 ± 0.184 |   4.941 |   5.302 |      1.0 |
| rangeops | 115.055 ± 7.302 | 107.261 | 121.737 |     22.6 |

### connect

```cmd
hyperfine --warmup 1 --export-markdown connect.md.tmp "linkr connect tests/Atha/clean.tsv > NUL" "rangeops connect tests/Atha/clean.tsv > NUL"

```

| Command  |     Mean [ms] | Min [ms] | Max [ms] | Relative |
| :------- | ------------: | -------: | -------: | -------: |
| linkr    |  320.8 ± 17.5 |    293.7 |    351.3 |      1.0 |
| rangeops | 5842.8 ± 63.4 |   5728.9 |   5930.4 |     18.2 |

### filter

```cmd
hyperfine --warmup 1 --export-markdown filter.md.tmp "linkr filter tests/Atha/connect.tsv -r 0.8 > NUL" "rangeops filter tests/Atha/connect.tsv -r 0.8 -o stdout > NUL"

```

| Command  |     Mean [ms] | Min [ms] | Max [ms] | Relative |
| :------- | ------------: | -------: | -------: | -------: |
| linkr    |    63.5 ± 5.7 |     55.2 |     78.1 |      1.0 |
| rangeops | 1423.2 ± 34.1 |   1385.7 |   1488.3 |     22.4 |
