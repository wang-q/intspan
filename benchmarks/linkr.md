# `linkr`, `jrange` and `rangeops`

```shell
brew install intspan
brew install jrange
cpanm App::Rangeops

brew install hyperfine

```

## `bash benchmarks/linkr/run.sh`

```shell
bash ~/Scripts/intspan/benchmarks/linkr/run.sh

rm ~/Scripts/intspan/benchmarks/linkr/*.tmp

```

### OSX 11.6 i7-8700k oracleJDK@18

```text
==> merge <==
==> jrange merge lastz blast
        2.89 real         4.13 user         0.21 sys
           652529664  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
              160994  page reclaims
                   0  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   1  signals received
                 118  voluntary context switches
                4371  involuntary context switches
         37410461948  instructions retired
         18487691396  cycles elapsed
           609824768  peak memory footprint
==> rgr merge lastz blast
        6.93 real         6.91 user         0.01 sys
             9736192  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
                2390  page reclaims
                   0  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                   0  voluntary context switches
                1669  involuntary context switches
         77317299730  instructions retired
         30035881697  cycles elapsed
             8495104  peak memory footprint
==> rangeops merge lastz blast
      114.52 real       308.76 user         1.20 sys
            69718016  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
              176132  page reclaims
                 548  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                 135  messages sent
                 126  messages received
                   0  signals received
                 453  voluntary context switches
              241748  involuntary context switches
         54025942898  instructions retired
         26236539133  cycles elapsed
            67203072  peak memory footprint

==> clean <==
==> jrange clean sort.clean
        2.27 real         3.81 user         0.24 sys
           509960192  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
              125953  page reclaims
                   0  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   1  signals received
                   2  voluntary context switches
                8710  involuntary context switches
         31436632868  instructions retired
         17123212271  cycles elapsed
           477802496  peak memory footprint
==> linkr clean sort.clean
        2.44 real         2.39 user         0.01 sys
            18178048  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
                4451  page reclaims
                   0  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                   0  voluntary context switches
                1916  involuntary context switches
         23088748720  instructions retired
         10302438203  cycles elapsed
            15097856  peak memory footprint
==> rangeops clean sort.clean
       52.79 real        52.39 user         0.22 sys
            73043968  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
               21691  page reclaims
                   0  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                   3  voluntary context switches
               30116  involuntary context switches
        468489165364  instructions retired
        225762118998  cycles elapsed
            70160384  peak memory footprint

==> clean bundle <==
==> jrange clean bundle sort.clean
        4.23 real         6.62 user         0.26 sys
           522530816  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
              129148  page reclaims
                   0  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   2  signals received
                   0  voluntary context switches
               17318  involuntary context switches
         59648631114  instructions retired
         29524446691  cycles elapsed
           482168832  peak memory footprint
==> linkr clean bundle sort.clean
        4.50 real         4.47 user         0.02 sys
            24215552  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
                5929  page reclaims
                   0  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                   0  voluntary context switches
                 963  involuntary context switches
         48248393033  instructions retired
         19244412966  cycles elapsed
            17686528  peak memory footprint
==> rangeops clean bundle sort.clean
       92.40 real        91.79 user         0.37 sys
            80662528  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
               23661  page reclaims
                   0  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                   4  voluntary context switches
               50365  involuntary context switches
        787407478040  instructions retired
        394885162910  cycles elapsed
            77492224  peak memory footprint

```

### Ubuntu 14.04 E5-2690 v3 openJDK@18

```text
==> merge <==
==> jrange merge lastz blast
	Command being timed: "jrange merge -o stdout -c 0.95 links.lastz.tsv links.blast.tsv"
	User time (seconds): 6.07
	System time (seconds): 0.44
	Percent of CPU this job got: 148%
	Elapsed (wall clock) time (h:mm:ss or m:ss): 0:04.38
	Average shared text size (kbytes): 0
	Average unshared data size (kbytes): 0
	Average stack size (kbytes): 0
	Average total size (kbytes): 0
	Maximum resident set size (kbytes): 1372452
	Average resident set size (kbytes): 0
	Major (requiring I/O) page faults: 0
	Minor (reclaiming a frame) page faults: 32177
	Voluntary context switches: 5228
	Involuntary context switches: 21
	Swaps: 0
	File system inputs: 0
	File system outputs: 0
	Socket messages sent: 0
	Socket messages received: 0
	Signals delivered: 0
	Page size (bytes): 4096
	Exit status: 0
==> rgr merge lastz blast
	Command being timed: "rgr merge -o stdout -c 0.95 links.lastz.tsv links.blast.tsv"
	User time (seconds): 4.36
	System time (seconds): 0.00
	Percent of CPU this job got: 99%
	Elapsed (wall clock) time (h:mm:ss or m:ss): 0:04.39
	Average shared text size (kbytes): 0
	Average unshared data size (kbytes): 0
	Average stack size (kbytes): 0
	Average total size (kbytes): 0
	Maximum resident set size (kbytes): 7504
	Average resident set size (kbytes): 0
	Major (requiring I/O) page faults: 0
	Minor (reclaiming a frame) page faults: 4450
	Voluntary context switches: 6
	Involuntary context switches: 3
	Swaps: 0
	File system inputs: 0
	File system outputs: 0
	Socket messages sent: 0
	Socket messages received: 0
	Signals delivered: 0
	Page size (bytes): 4096
	Exit status: 0
==> rangeops merge lastz blast
	Command being timed: "rangeops merge -o stdout -c 0.95 -p 8 links.lastz.tsv links.blast.tsv"
	User time (seconds): 394.96
	System time (seconds): 0.90
	Percent of CPU this job got: 269%
	Elapsed (wall clock) time (h:mm:ss or m:ss): 2:26.69
	Average shared text size (kbytes): 0
	Average unshared data size (kbytes): 0
	Average stack size (kbytes): 0
	Average total size (kbytes): 0
	Maximum resident set size (kbytes): 72340
	Average resident set size (kbytes): 0
	Major (requiring I/O) page faults: 0
	Minor (reclaiming a frame) page faults: 386079
	Voluntary context switches: 13292
	Involuntary context switches: 732
	Swaps: 0
	File system inputs: 0
	File system outputs: 0
	Socket messages sent: 0
	Socket messages received: 0
	Signals delivered: 0
	Page size (bytes): 4096
	Exit status: 0

==> clean <==
==> jrange clean sort.clean
	Command being timed: "jrange clean -o stdout sort.clean.tsv"
	User time (seconds): 5.17
	System time (seconds): 0.48
	Percent of CPU this job got: 194%
	Elapsed (wall clock) time (h:mm:ss or m:ss): 0:02.91
	Average shared text size (kbytes): 0
	Average unshared data size (kbytes): 0
	Average stack size (kbytes): 0
	Average total size (kbytes): 0
	Maximum resident set size (kbytes): 1378808
	Average resident set size (kbytes): 0
	Major (requiring I/O) page faults: 0
	Minor (reclaiming a frame) page faults: 34077
	Voluntary context switches: 7007
	Involuntary context switches: 15
	Swaps: 0
	File system inputs: 0
	File system outputs: 0
	Socket messages sent: 0
	Socket messages received: 0
	Signals delivered: 0
	Page size (bytes): 4096
	Exit status: 0
==> linkr clean sort.clean
	Command being timed: "linkr clean -o stdout sort.clean.tsv"
	User time (seconds): 1.22
	System time (seconds): 0.01
	Percent of CPU this job got: 99%
	Elapsed (wall clock) time (h:mm:ss or m:ss): 0:01.23
	Average shared text size (kbytes): 0
	Average unshared data size (kbytes): 0
	Average stack size (kbytes): 0
	Average total size (kbytes): 0
	Maximum resident set size (kbytes): 11388
	Average resident set size (kbytes): 0
	Major (requiring I/O) page faults: 0
	Minor (reclaiming a frame) page faults: 3783
	Voluntary context switches: 3
	Involuntary context switches: 2
	Swaps: 0
	File system inputs: 0
	File system outputs: 0
	Socket messages sent: 0
	Socket messages received: 0
	Signals delivered: 0
	Page size (bytes): 4096
	Exit status: 0
==> rangeops clean sort.clean
	Command being timed: "rangeops clean -o stdout sort.clean.tsv"
	User time (seconds): 64.05
	System time (seconds): 0.12
	Percent of CPU this job got: 100%
	Elapsed (wall clock) time (h:mm:ss or m:ss): 1:04.17
	Average shared text size (kbytes): 0
	Average unshared data size (kbytes): 0
	Average stack size (kbytes): 0
	Average total size (kbytes): 0
	Maximum resident set size (kbytes): 75796
	Average resident set size (kbytes): 0
	Major (requiring I/O) page faults: 0
	Minor (reclaiming a frame) page faults: 62153
	Voluntary context switches: 32
	Involuntary context switches: 54
	Swaps: 0
	File system inputs: 0
	File system outputs: 0
	Socket messages sent: 0
	Socket messages received: 0
	Signals delivered: 0
	Page size (bytes): 4096
	Exit status: 0

==> clean bundle <==
==> jrange clean bundle sort.clean
	Command being timed: "jrange clean -o stdout --bundle 500 sort.clean.tsv"
	User time (seconds): 8.76
	System time (seconds): 0.55
	Percent of CPU this job got: 157%
	Elapsed (wall clock) time (h:mm:ss or m:ss): 0:05.93
	Average shared text size (kbytes): 0
	Average unshared data size (kbytes): 0
	Average stack size (kbytes): 0
	Average total size (kbytes): 0
	Maximum resident set size (kbytes): 1375340
	Average resident set size (kbytes): 0
	Major (requiring I/O) page faults: 0
	Minor (reclaiming a frame) page faults: 35715
	Voluntary context switches: 7075
	Involuntary context switches: 18
	Swaps: 0
	File system inputs: 0
	File system outputs: 0
	Socket messages sent: 0
	Socket messages received: 0
	Signals delivered: 0
	Page size (bytes): 4096
	Exit status: 0
==> linkr clean bundle sort.clean
	Command being timed: "linkr clean -o stdout --bundle 500 sort.clean.tsv"
	User time (seconds): 2.76
	System time (seconds): 0.01
	Percent of CPU this job got: 99%
	Elapsed (wall clock) time (h:mm:ss or m:ss): 0:02.78
	Average shared text size (kbytes): 0
	Average unshared data size (kbytes): 0
	Average stack size (kbytes): 0
	Average total size (kbytes): 0
	Maximum resident set size (kbytes): 12644
	Average resident set size (kbytes): 0
	Major (requiring I/O) page faults: 0
	Minor (reclaiming a frame) page faults: 6245
	Voluntary context switches: 3
	Involuntary context switches: 11
	Swaps: 0
	File system inputs: 0
	File system outputs: 0
	Socket messages sent: 0
	Socket messages received: 0
	Signals delivered: 0
	Page size (bytes): 4096
	Exit status: 0
==> rangeops clean bundle sort.clean
	Command being timed: "rangeops clean -o stdout --bundle 500 sort.clean.tsv"
	User time (seconds): 111.82
	System time (seconds): 0.13
	Percent of CPU this job got: 100%
	Elapsed (wall clock) time (h:mm:ss or m:ss): 1:51.96
	Average shared text size (kbytes): 0
	Average unshared data size (kbytes): 0
	Average stack size (kbytes): 0
	Average total size (kbytes): 0
	Maximum resident set size (kbytes): 80944
	Average resident set size (kbytes): 0
	Major (requiring I/O) page faults: 0
	Minor (reclaiming a frame) page faults: 99363
	Voluntary context switches: 33
	Involuntary context switches: 57
	Swaps: 0
	File system inputs: 0
	File system outputs: 0
	Socket messages sent: 0
	Socket messages received: 0
	Signals delivered: 0
	Page size (bytes): 4096
	Exit status: 0

```

## `linkr`

* macOS 10.14
    * i7-8700k
    * oracleJDK8
    * rustc 1.37.0
    * Perl 5.30.0
* Windows 11 WSL
    * Ryzen 7 5800
    * openJDK 18.0.1
    * rustc 1.60.0
    * Perl 5.34.0

```shell
rm linkr.all.md.tmp

hyperfine --warmup 1 --export-markdown linkr.md.tmp \
    -n "sort linkr" \
    -n "sort jrange" \
    -n "sort rangeops" \
    'gzip -dcf tests/Atha/links.lastz.tsv.gz tests/Atha/links.blast.tsv.gz | linkr    sort stdin -o /dev/null' \
    'gzip -dcf tests/Atha/links.lastz.tsv.gz tests/Atha/links.blast.tsv.gz | jrange   sort stdin -o /dev/null' \
    'gzip -dcf tests/Atha/links.lastz.tsv.gz tests/Atha/links.blast.tsv.gz | rangeops sort stdin -o /dev/null'

cat linkr.md.tmp |
    ( cat && echo ) \
    >> linkr.all.md.tmp

hyperfine --min-runs 3 --export-markdown linkr.md.tmp \
    -n "clean linkr" \
    -n "clean jrange" \
    -n "clean rangeops" \
    'linkr    clean tests/Atha/sort.tsv -o /dev/null' \
    'jrange   clean tests/Atha/sort.tsv -o /dev/null' \
    'rangeops clean tests/Atha/sort.tsv -o /dev/null'

cat linkr.md.tmp |
    ( cat && echo ) \
    >> linkr.all.md.tmp

hyperfine --min-runs 3 --export-markdown linkr.md.tmp \
    -n "merge rgr" \
    -n "merge jrange" \
    -n "merge rangeops" \
    'rgr      merge tests/Atha/sort.clean.tsv -c 0.95 -o /dev/null' \
    'jrange   merge tests/Atha/sort.clean.tsv -c 0.95 -o /dev/null' \
    'rangeops merge tests/Atha/sort.clean.tsv -c 0.95 -p 4 -o /dev/null'

cat linkr.md.tmp |
    ( cat && echo ) \
    >> linkr.all.md.tmp

hyperfine --min-runs 3 --export-markdown linkr.md.tmp \
    -n "clean2 linkr" \
    -n "clean2 jrange" \
    -n "clean2 rangeops" \
    'linkr    clean tests/Atha/sort.clean.tsv -r tests/Atha/merge.tsv --bundle 500 -o /dev/null' \
    'jrange   clean tests/Atha/sort.clean.tsv -r tests/Atha/merge.tsv --bundle 500 -o /dev/null' \
    'rangeops clean tests/Atha/sort.clean.tsv -r tests/Atha/merge.tsv --bundle 500 -o /dev/null'

cat linkr.md.tmp |
    ( cat && echo ) \
    >> linkr.all.md.tmp

hyperfine --export-markdown linkr.md.tmp \
    -n "connect linkr" \
    -n "connect rangeops" \
    'linkr    connect tests/Atha/clean.tsv -o /dev/null' \
    'rangeops connect tests/Atha/clean.tsv -o /dev/null'

cat linkr.md.tmp |
    ( cat && echo ) \
    >> linkr.all.md.tmp

hyperfine --warmup 1 --export-markdown linkr.md.tmp \
    -n "filter linkr" \
    -n "filter jrange" \
    -n "filter rangeops" \
    'linkr    filter tests/Atha/connect.tsv -r 0.8 -o /dev/null' \
    'jrange   filter tests/Atha/connect.tsv -r 0.8 -o /dev/null' \
    'rangeops filter tests/Atha/connect.tsv -r 0.8 -o /dev/null'

cat linkr.md.tmp |
    ( cat && echo ) \
    >> linkr.all.md.tmp

cat linkr.all.md.tmp

```

### i7-4770HQ macOS Big Sur

| Command         |      Mean [ms] | Min [ms] | Max [ms] |     Relative |
|:----------------|---------------:|---------:|---------:|-------------:|
| `sort linkr`    |   192.5 ± 21.6 |    169.8 |    243.3 |         1.00 |
| `sort jrange`   | 1396.4 ± 221.2 |   1194.8 |   1795.4 |  7.26 ± 1.41 |
| `sort rangeops` | 2718.7 ± 698.2 |   2045.9 |   4508.8 | 14.13 ± 3.96 |

| Command          |        Mean [s] | Min [s] | Max [s] |     Relative |
|:-----------------|----------------:|--------:|--------:|-------------:|
| `clean linkr`    |  10.237 ± 0.929 |   9.283 |  11.139 |         1.00 |
| `clean jrange`   |  10.926 ± 1.499 |  10.018 |  12.656 |  1.07 ± 0.18 |
| `clean rangeops` | 217.755 ± 9.570 | 206.715 | 223.689 | 21.27 ± 2.14 |

| Command          |        Mean [s] | Min [s] | Max [s] |     Relative |
|:-----------------|----------------:|--------:|--------:|-------------:|
| `merge rgr`      |  10.334 ± 1.050 |   9.211 |  11.292 |  2.97 ± 0.30 |
| `merge jrange`   |   3.484 ± 0.038 |   3.457 |   3.527 |         1.00 |
| `merge rangeops` | 133.528 ± 3.098 | 130.136 | 136.208 | 38.33 ± 0.98 |

| Command           |        Mean [s] | Min [s] | Max [s] |     Relative |
|:------------------|----------------:|--------:|--------:|-------------:|
| `clean2 linkr`    |   4.505 ± 0.283 |   4.182 |   4.704 |         1.00 |
| `clean2 jrange`   |   7.109 ± 0.094 |   7.033 |   7.215 |  1.58 ± 0.10 |
| `clean2 rangeops` | 104.708 ± 4.599 |  99.539 | 108.345 | 23.24 ± 1.78 |

| Command            |        Mean [ms] | Min [ms] | Max [ms] |      Relative |
|:-------------------|-----------------:|---------:|---------:|--------------:|
| `connect linkr`    |     274.5 ± 12.3 |    257.4 |    295.4 |          1.00 |
| `connect rangeops` | 37331.9 ± 1713.8 |  34847.7 |  39465.5 | 135.99 ± 8.71 |

| Command           |    Mean [ms] | Min [ms] | Max [ms] |     Relative |
|:------------------|-------------:|---------:|---------:|-------------:|
| `filter linkr`    |   45.8 ± 3.6 |     41.3 |     57.6 |         1.00 |
| `filter jrange`   | 186.1 ± 26.4 |    156.9 |    246.5 |  4.06 ± 0.66 |
| `filter rangeops` | 725.4 ± 90.3 |    633.3 |    943.5 | 15.83 ± 2.34 |

## `linkr` on Windows

* Ryzen 7 5800
* Windows 11 21H2
* rustc 1.62.0 msvc
* Strawberry Perl 5.32.1

### sort

```powershell
hyperfine --warmup 1 --export-markdown sort.md.tmp `
    -n linkr `
    -n rangeops `
    "gzip -dcf tests/Atha/links.lastz.tsv.gz tests/Atha/links.blast.tsv.gz | linkr sort stdin > NUL" `
    "gzip -dcf tests/Atha/links.lastz.tsv.gz tests/Atha/links.blast.tsv.gz | rangeops sort stdin -o stdout > NUL"

cat sort.md.tmp

```

| Command    |    Mean [ms] | Min [ms] | Max [ms] |     Relative |
|:-----------|-------------:|---------:|---------:|-------------:|
| `linkr`    |  107.9 ± 0.9 |    106.2 |    109.8 |         1.00 |
| `rangeops` | 1263.4 ± 9.3 |   1249.0 |   1279.3 | 11.71 ± 0.13 |

### clean

```powershell
hyperfine --min-runs 3 --export-markdown clean.md.tmp `
    -n linkr `
    -n rangeops `
    "linkr clean tests/Atha/sort.tsv > NUL" `
    "rangeops clean tests/Atha/sort.tsv -o stdout > NUL"

cat clean.md.tmp

```

| Command    |       Mean [s] | Min [s] | Max [s] |     Relative |
|:-----------|---------------:|--------:|--------:|-------------:|
| `linkr`    |  2.306 ± 0.065 |   2.253 |   2.379 |         1.00 |
| `rangeops` | 91.878 ± 0.535 |  91.265 |  92.254 | 39.84 ± 1.15 |

### merge

```powershell
hyperfine --min-runs 3 --export-markdown merge.md.tmp `
    -n rgr `
    -n rangeops `
    "rgr merge tests/Atha/sort.clean.tsv -c 0.95 > NUL" `
    "rangeops merge tests/Atha/sort.clean.tsv -c 0.95 -p 4 -o stdout > NUL"

cat merge.md.tmp

```

| Command    |       Mean [s] | Min [s] | Max [s] |     Relative |
|:-----------|---------------:|--------:|--------:|-------------:|
| `rgr`      |  2.532 ± 0.030 |   2.513 |   2.566 |         1.00 |
| `rangeops` | 54.278 ± 1.479 |  52.890 |  55.833 | 21.44 ± 0.64 |

### clean2

```powershell
hyperfine --min-runs 3 --export-markdown clean2.md.tmp `
    -n linkr `
    -n rangeops `
    "linkr clean tests/Atha/sort.clean.tsv -r tests/Atha/merge.tsv --bundle 500 > NUL" `
    "rangeops clean tests/Atha/sort.clean.tsv -r tests/Atha/merge.tsv --bundle 500 > NUL"

cat clean2.md.tmp

```

| Command    |       Mean [s] | Min [s] | Max [s] |     Relative |
|:-----------|---------------:|--------:|--------:|-------------:|
| `linkr`    |  1.316 ± 0.024 |   1.294 |   1.342 |         1.00 |
| `rangeops` | 44.935 ± 0.419 |  44.478 |  45.303 | 34.16 ± 0.71 |

### connect

```powershell
hyperfine --warmup 1 --export-markdown connect.md.tmp `
    -n linkr `
    -n rangeops `
    "linkr connect tests/Atha/clean.tsv > NUL" `
    "rangeops connect tests/Atha/clean.tsv > NUL"

cat connect.md.tmp

```

| Command    |       Mean [ms] | Min [ms] | Max [ms] |       Relative |
|:-----------|----------------:|---------:|---------:|---------------:|
| `linkr`    |    114.6 ± 12.5 |    106.9 |    158.4 |           1.00 |
| `rangeops` | 14018.0 ± 122.7 |  13872.1 |  14239.5 | 122.36 ± 13.42 |

### filter

```powershell
hyperfine --warmup 1 --export-markdown filter.md.tmp `
    -n linkr `
    -n rangeops `
    "linkr filter tests/Atha/connect.tsv -r 0.8 > NUL" `
    "rangeops filter tests/Atha/connect.tsv -r 0.8 -o stdout > NUL"

cat filter.md.tmp

```

| Command    |    Mean [ms] | Min [ms] | Max [ms] |     Relative |
|:-----------|-------------:|---------:|---------:|-------------:|
| `linkr`    |   22.2 ± 0.6 |     20.9 |     24.5 |         1.00 |
| `rangeops` | 481.6 ± 29.5 |    462.4 |    562.0 | 21.65 ± 1.46 |
