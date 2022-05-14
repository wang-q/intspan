# `linkr`, `jrange` and `rangeops`

```bash
brew install intspan
brew install jrange
cpanm App::Rangeops

brew install hyperfine

```

## `bash benchmarks/linkr/run.sh`

```bash
bash ~/Scripts/intspan/benchmarks/linkr/run.sh

rm ~/Scripts/intspan/benchmarks/linkr/*.tmp

```

* OSX 11.6 i7-8700k oracleJDK@18

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

* Ubuntu 14.04 E5-2690 v3 openJDK@18

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
    'rgr      merge tests/Atha/sort.clean.tsv -c 0.95 -o /dev/null' \
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

* Ryzen 7 PRO 3700U
* Windows 10 19041
* Strawberry Perl 5.30.2.1
* rustc 1.42.0 msvc

### sort

```ps1
hyperfine --warmup 1 --export-markdown sort.md.tmp `
    "gzip -dcf tests/Atha/links.lastz.tsv.gz tests/Atha/links.blast.tsv.gz | linkr sort stdin > nul " `
    "gzip -dcf tests/Atha/links.lastz.tsv.gz tests/Atha/links.blast.tsv.gz | rangeops sort stdin -o stdout > nul "

```

| Command  |     Mean [ms] | Min [ms] | Max [ms] |     Relative |
|:---------|--------------:|---------:|---------:|-------------:|
| linkr    |  248.5 ± 12.5 |    230.9 |    277.6 |         1.00 |
| rangeops | 3472.4 ± 39.9 |   3420.0 |   3532.1 | 13.97 ± 0.72 |

### clean

```ps1
hyperfine --min-runs 3 --export-markdown clean.md.tmp `
    "linkr clean tests/Atha/sort.tsv > NUL" `
    "rangeops clean tests/Atha/sort.tsv -o stdout > NUL"

```

| Command  |        Mean [s] | Min [s] | Max [s] |     Relative |
|:---------|----------------:|--------:|--------:|-------------:|
| linkr    |   9.062 ± 0.179 |   8.856 |   9.175 |         1.00 |
| rangeops | 244.335 ± 2.320 | 242.105 | 246.736 | 26.96 ± 0.59 |

### merge

```ps1
hyperfine --min-runs 3 --export-markdown merge.md.tmp `
    "rgr merge tests/Atha/sort.clean.tsv -c 0.95 > NUL" `
    "rangeops merge tests/Atha/sort.clean.tsv -c 0.95 -p 4 -o stdout > NUL"

```

| Command  |        Mean [s] | Min [s] | Max [s] |     Relative |
|:---------|----------------:|--------:|--------:|-------------:|
| linkr    |   9.356 ± 0.237 |   9.092 |   9.551 |         1.00 |
| rangeops | 156.579 ± 1.006 | 155.680 | 157.665 | 16.74 ± 0.44 |

### clean2

```ps1
hyperfine --min-runs 3 --export-markdown clean2.md.tmp `
    "linkr clean tests/Atha/sort.clean.tsv -r tests/Atha/merge.tsv --bundle 500 > NUL" `
    "rangeops clean tests/Atha/sort.clean.tsv -r tests/Atha/merge.tsv --bundle 500 > NUL"

```

| Command  |        Mean [s] | Min [s] | Max [s] |     Relative |
|:---------|----------------:|--------:|--------:|-------------:|
| linkr    |   4.466 ± 0.040 |   4.443 |   4.512 |         1.00 |
| rangeops | 109.768 ± 0.774 | 108.875 | 110.243 | 24.58 ± 0.28 |

### connect

```ps1
hyperfine --warmup 1 --export-markdown connect.md.tmp `
    "linkr connect tests/Atha/clean.tsv > NUL" `
    "rangeops connect tests/Atha/clean.tsv > NUL"

```

| Command  |     Mean [ms] | Min [ms] | Max [ms] |     Relative |
|:---------|--------------:|---------:|---------:|-------------:|
| linkr    |   284.1 ± 9.2 |    272.6 |    303.1 |         1.00 |
| rangeops | 5578.0 ± 46.4 |   5524.2 |   5692.2 | 19.63 ± 0.65 |

### filter

```cmd
hyperfine --warmup 1 --export-markdown filter.md.tmp `
    "linkr filter tests/Atha/connect.tsv -r 0.8 > NUL" `
    "rangeops filter tests/Atha/connect.tsv -r 0.8 -o stdout > NUL"

```

| Command  |     Mean [ms] | Min [ms] | Max [ms] |     Relative |
|:---------|--------------:|---------:|---------:|-------------:|
| linkr    |    56.7 ± 7.5 |     48.6 |     80.4 |         1.00 |
| rangeops | 1366.2 ± 26.4 |   1315.2 |   1413.9 | 24.12 ± 3.24 |
