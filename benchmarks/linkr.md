# `linkr`, `jrange` and `rangeops`

```bash
brew install intspan
brew install jrange
cpanm App::Rangeops

brew install hyperfine

```

## `bash benchmarks/linkr/run.sh`

```bash
bash ~/Scripts/rust/intspan/benchmarks/linkr/run.sh

rm ~/Scripts/rust/intspan/benchmarks/linkr/*.tmp

```

* OSX 10.14 i7-8700k oracleJDK8

```text
==> merge <==
==> jrange merge lastz blast
        3.53 real         4.39 user         0.41 sys
1161953280  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
    285816  page reclaims
         2  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         1  signals received
       116  voluntary context switches
      5458  involuntary context switches
==> linkr merge lastz blast
        8.21 real         8.14 user         0.03 sys
  10682368  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
      2617  page reclaims
         0  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         0  signals received
         0  voluntary context switches
      7349  involuntary context switches
==> rangeops merge lastz blast
      114.75 real       317.46 user         1.76 sys
  81735680  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
    194485  page reclaims
         0  page faults
         0  swaps
         0  block input operations
         0  block output operations
       140  messages sent
       131  messages received
         0  signals received
       144  voluntary context switches
    371802  involuntary context switches

==> clean <==
==> jrange clean sort.clean
        2.51 real         3.75 user         0.43 sys
1152802816  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
    283479  page reclaims
         0  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         1  signals received
        16  voluntary context switches
      6481  involuntary context switches
==> linkr clean sort.clean
        2.59 real         2.56 user         0.01 sys
  17092608  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
      4182  page reclaims
         0  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         0  signals received
         0  voluntary context switches
      1077  involuntary context switches
==> rangeops clean sort.clean
       54.43 real        54.09 user         0.20 sys
  84979712  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
     25085  page reclaims
         0  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         0  signals received
         5  voluntary context switches
     24855  involuntary context switches

==> clean bundle <==
==> jrange clean bundle sort.clean
        4.80 real         6.57 user         0.46 sys
1163481088  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
    286496  page reclaims
         0  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         3  signals received
         2  voluntary context switches
      9727  involuntary context switches
==> linkr clean bundle sort.clean
        5.17 real         5.11 user         0.02 sys
  26021888  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
      6058  page reclaims
       304  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         0  signals received
         2  voluntary context switches
      3377  involuntary context switches
==> rangeops clean bundle sort.clean
       85.37 real        84.86 user         0.27 sys
  89972736  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
     26372  page reclaims
        44  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         0  signals received
       251  voluntary context switches
     31324  involuntary context switches

```

## `linkr` on macOS

* macOS 10.14
* i7-8700k
* oracleJDK8
* rustc 1.37.0
* Perl 5.30.0

### sort

```bash
hyperfine --warmup 1 --export-markdown sort.md.tmp \
    'gzip -dcf tests/Atha/links.lastz.tsv.gz tests/Atha/links.blast.tsv.gz | linkr    sort stdin -o /dev/null' \
    'gzip -dcf tests/Atha/links.lastz.tsv.gz tests/Atha/links.blast.tsv.gz | jrange   sort stdin -o /dev/null' \
    'gzip -dcf tests/Atha/links.lastz.tsv.gz tests/Atha/links.blast.tsv.gz | rangeops sort stdin -o /dev/null'

```

| Command  |     Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---------|--------------:|---------:|---------:|---------:|
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
|:---------|----------------:|--------:|--------:|---------:|
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
|:---------|---------------:|--------:|--------:|---------:|
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
|:---------|---------------:|--------:|--------:|---------:|
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
|:---------|--------------:|---------:|---------:|---------:|
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
|:---------|------------:|---------:|---------:|---------:|
| linkr    |  31.0 ± 0.6 |     30.1 |     33.0 |      1.0 |
| jrange   | 109.6 ± 1.9 |    105.4 |    114.2 |      3.5 |
| rangeops | 415.7 ± 6.4 |    409.3 |    429.2 |     13.4 |

## `linkr` on Windows

* E3-1245 V2
* Windows 10 18950
* strawberry-perl-5.30.0.1-64bit
* rustc 1.40.0 msvc

### sort

```cmd
hyperfine --warmup 1 --export-markdown sort.md.tmp "gzip -dcf tests/Atha/links.lastz.tsv.gz tests/Atha/links.blast.tsv.gz | linkr sort stdin > NUL" "gzip -dcf tests/Atha/links.lastz.tsv.gz tests/Atha/links.blast.tsv.gz | rangeops sort stdin -o stdout > NUL"

```

| Command  |     Mean [ms] | Min [ms] | Max [ms] |     Relative |
|:---------|--------------:|---------:|---------:|-------------:|
| linkr    |  248.5 ± 12.5 |    230.9 |    277.6 |         1.00 |
| rangeops | 3472.4 ± 39.9 |   3420.0 |   3532.1 | 13.97 ± 0.72 |

### clean

```cmd
hyperfine --min-runs 3 --export-markdown clean.md.tmp "linkr clean tests/Atha/sort.tsv > NUL" "rangeops clean tests/Atha/sort.tsv -o stdout > NUL"

```

| Command  |        Mean [s] | Min [s] | Max [s] |     Relative |
|:---------|----------------:|--------:|--------:|-------------:|
| linkr    |   9.062 ± 0.179 |   8.856 |   9.175 |         1.00 |
| rangeops | 244.335 ± 2.320 | 242.105 | 246.736 | 26.96 ± 0.59 |

### merge

```cmd
hyperfine --min-runs 3 --export-markdown merge.md.tmp "linkr merge tests/Atha/sort.clean.tsv -c 0.95 > NUL" "rangeops merge tests/Atha/sort.clean.tsv -c 0.95 -p 4 -o stdout > NUL"

```

| Command  |        Mean [s] | Min [s] | Max [s] |     Relative |
|:---------|----------------:|--------:|--------:|-------------:|
| linkr    |   9.356 ± 0.237 |   9.092 |   9.551 |         1.00 |
| rangeops | 156.579 ± 1.006 | 155.680 | 157.665 | 16.74 ± 0.44 |

### clean2

```cmd
hyperfine --min-runs 3 --export-markdown clean2.md.tmp "linkr clean tests/Atha/sort.clean.tsv -r tests/Atha/merge.tsv --bundle 500 > NUL" "rangeops clean tests/Atha/sort.clean.tsv -r tests/Atha/merge.tsv --bundle 500 > NUL"

```

| Command  |        Mean [s] | Min [s] | Max [s] |     Relative |
|:---------|----------------:|--------:|--------:|-------------:|
| linkr    |   4.466 ± 0.040 |   4.443 |   4.512 |         1.00 |
| rangeops | 109.768 ± 0.774 | 108.875 | 110.243 | 24.58 ± 0.28 |

### connect

```cmd
hyperfine --warmup 1 --export-markdown connect.md.tmp "linkr connect tests/Atha/clean.tsv > NUL" "rangeops connect tests/Atha/clean.tsv > NUL"

```

| Command  |     Mean [ms] | Min [ms] | Max [ms] |     Relative |
|:---------|--------------:|---------:|---------:|-------------:|
| linkr    |   284.1 ± 9.2 |    272.6 |    303.1 |         1.00 |
| rangeops | 5578.0 ± 46.4 |   5524.2 |   5692.2 | 19.63 ± 0.65 |

### filter

```cmd
hyperfine --warmup 1 --export-markdown filter.md.tmp "linkr filter tests/Atha/connect.tsv -r 0.8 > NUL" "rangeops filter tests/Atha/connect.tsv -r 0.8 -o stdout > NUL"

```

| Command  |     Mean [ms] | Min [ms] | Max [ms] |     Relative |
|:---------|--------------:|---------:|---------:|-------------:|
| linkr    |    56.7 ± 7.5 |     48.6 |     80.4 |         1.00 |
| rangeops | 1366.2 ± 26.4 |   1315.2 |   1413.9 | 24.12 ± 3.24 |
