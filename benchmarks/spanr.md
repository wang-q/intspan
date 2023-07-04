# `spanr`, `jrunlist` and `rangeops`

```shell
brew install intspan
brew install jrunlist
cpanm App::RL

brew install hyperfine
brew install faops

```

## `bash benchmarks/spanr/run.sh`

```shell
bash ~/Scripts/intspan/benchmarks/spanr/run.sh

rm ~/Scripts/intspan/benchmarks/spanr/*.tmp

```

### i7-8700k OSX 10.14 oracleJDK8

```text
==> jrunlist
        3.23 real         8.72 user         1.35 sys
1051443200  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
    262964  page reclaims
        68  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         2  signals received
       415  voluntary context switches
     70319  involuntary context switches
==> spanr
        1.96 real         1.89 user         0.04 sys
 109182976  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
     26425  page reclaims
       246  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         0  signals received
         1  voluntary context switches
       238  involuntary context switches
==> App::RL
      217.36 real       216.89 user         0.25 sys
 118370304  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
     28895  page reclaims
        13  page faults
         0  swaps
         0  block input operations
         0  block output operations
         0  messages sent
         0  messages received
         0  signals received
        18  voluntary context switches
     37340  involuntary context switches

```

### R5 4600U Windows 11 WSL openJDK@17

```text
==> jrunlist
        Command being timed: "jrunlist statop chr.sizes sep-gene.yml paralog.json --op intersect --all -o stdout"
        User time (seconds): 8.48
        System time (seconds): 1.55
        Percent of CPU this job got: 123%
        Elapsed (wall clock) time (h:mm:ss or m:ss): 0:08.10
        Average shared text size (kbytes): 0
        Average unshared data size (kbytes): 0
        Average stack size (kbytes): 0
        Average total size (kbytes): 0
        Maximum resident set size (kbytes): 967152
        Average resident set size (kbytes): 0
        Major (requiring I/O) page faults: 12
        Minor (reclaiming a frame) page faults: 40607
        Voluntary context switches: 35163
        Involuntary context switches: 27
        Swaps: 0
        File system inputs: 0
        File system outputs: 88
        Socket messages sent: 0
        Socket messages received: 0
        Signals delivered: 0
        Page size (bytes): 4096
        Exit status: 0
==> spanr
        Command being timed: "spanr statop chr.sizes sep-gene.yml paralog.json --op intersect --all -o stdout"
        User time (seconds): 1.83
        System time (seconds): 0.05
        Percent of CPU this job got: 98%
        Elapsed (wall clock) time (h:mm:ss or m:ss): 0:01.91
        Average shared text size (kbytes): 0
        Average unshared data size (kbytes): 0
        Average stack size (kbytes): 0
        Average total size (kbytes): 0
        Maximum resident set size (kbytes): 99244
        Average resident set size (kbytes): 0
        Major (requiring I/O) page faults: 0
        Minor (reclaiming a frame) page faults: 26578
        Voluntary context switches: 262
        Involuntary context switches: 4
        Swaps: 0
        File system inputs: 0
        File system outputs: 0
        Socket messages sent: 0
        Socket messages received: 0
        Signals delivered: 0
        Page size (bytes): 4096
        Exit status: 0
==> App::RL
        Command being timed: "runlist stat2 -s chr.sizes sep-gene.yml paralog.json --op intersect --all --mk -o stdout"
        User time (seconds): 284.90
        System time (seconds): 0.10
        Percent of CPU this job got: 99%
        Elapsed (wall clock) time (h:mm:ss or m:ss): 4:45.05
        Average shared text size (kbytes): 0
        Average unshared data size (kbytes): 0
        Average stack size (kbytes): 0
        Average total size (kbytes): 0
        Maximum resident set size (kbytes): 117052
        Average resident set size (kbytes): 0
        Major (requiring I/O) page faults: 0
        Minor (reclaiming a frame) page faults: 28987
        Voluntary context switches: 419
        Involuntary context switches: 589
        Swaps: 0
        File system inputs: 0
        File system outputs: 0
        Socket messages sent: 0
        Socket messages received: 0
        Signals delivered: 0
        Page size (bytes): 4096
        Exit status: 0

```

### E5-2690 v3 Ubuntu 14.04 openJDK@8

```text
==> jrunlist
	Command being timed: "jrunlist statop chr.sizes sep-gene.yml paralog.json --op intersect --all -o stdout"
	User time (seconds): 7.31
	System time (seconds): 0.88
	Percent of CPU this job got: 274%
	Elapsed (wall clock) time (h:mm:ss or m:ss): 0:02.98
	Average shared text size (kbytes): 0
	Average unshared data size (kbytes): 0
	Average stack size (kbytes): 0
	Average total size (kbytes): 0
	Maximum resident set size (kbytes): 1342940
	Average resident set size (kbytes): 0
	Major (requiring I/O) page faults: 0
	Minor (reclaiming a frame) page faults: 36288
	Voluntary context switches: 7952
	Involuntary context switches: 25
	Swaps: 0
	File system inputs: 0
	File system outputs: 0
	Socket messages sent: 0
	Socket messages received: 0
	Signals delivered: 0
	Page size (bytes): 4096
	Exit status: 0
==> spanr
	Command being timed: "spanr statop chr.sizes sep-gene.yml paralog.json --op intersect --all -o stdout"
	User time (seconds): 1.59
	System time (seconds): 0.05
	Percent of CPU this job got: 99%
	Elapsed (wall clock) time (h:mm:ss or m:ss): 0:01.65
	Average shared text size (kbytes): 0
	Average unshared data size (kbytes): 0
	Average stack size (kbytes): 0
	Average total size (kbytes): 0
	Maximum resident set size (kbytes): 84548
	Average resident set size (kbytes): 0
	Major (requiring I/O) page faults: 0
	Minor (reclaiming a frame) page faults: 23559
	Voluntary context switches: 4
	Involuntary context switches: 3
	Swaps: 0
	File system inputs: 0
	File system outputs: 0
	Socket messages sent: 0
	Socket messages received: 0
	Signals delivered: 0
	Page size (bytes): 4096
	Exit status: 0
==> App::RL
	Command being timed: "runlist stat2 -s chr.sizes sep-gene.yml paralog.json --op intersect --all --mk -o stdout"
	User time (seconds): 321.30
	System time (seconds): 0.20
	Percent of CPU this job got: 100%
	Elapsed (wall clock) time (h:mm:ss or m:ss): 5:21.50
	Average shared text size (kbytes): 0
	Average unshared data size (kbytes): 0
	Average stack size (kbytes): 0
	Average total size (kbytes): 0
	Maximum resident set size (kbytes): 114400
	Average resident set size (kbytes): 0
	Major (requiring I/O) page faults: 0
	Minor (reclaiming a frame) page faults: 151398
	Voluntary context switches: 25
	Involuntary context switches: 85
	Swaps: 0
	File system inputs: 0
	File system outputs: 0
	Socket messages sent: 0
	Socket messages received: 0
	Signals delivered: 0
	Page size (bytes): 4096
	Exit status: 0

```

## `spanr cover`

```shell
mkdir -p ~/data/intspan
cd ~/data/intspan

curl -o S288c.fa.gz \
    -L http://ftp.ensemblgenomes.org/pub/fungi/release-53/fasta/saccharomyces_cerevisiae/dna/Saccharomyces_cerevisiae.R64-1-1.dna_sm.toplevel.fa.gz

curl -o Spom.fa.gz \
    -L http://ftp.ensemblgenomes.org/pub/fungi/release-53/fasta/schizosaccharomyces_pombe/dna/Schizosaccharomyces_pombe.ASM294v2.dna_sm.toplevel.fa.gz

faops masked S288c.fa.gz > S288c.rg
faops masked Spom.fa.gz > Spom.rg

faops size S288c.fa.gz > S288c.chr.sizes
faops size Spom.fa.gz > Spom.chr.sizes

```

```shell
cd ~/data/intspan

rm cover.all.md.tmp

hyperfine --warmup 1 --export-markdown cover.md.tmp \
    -n spanr \
    -n jrunlist \
    -n runlist \
    'faops masked S288c.fa.gz | spanr    cover stdin -o /dev/null' \
    'faops masked S288c.fa.gz | jrunlist cover stdin -o /dev/null' \
    'faops masked S288c.fa.gz | runlist  cover stdin -o /dev/null'

cat cover.md.tmp |
    ( cat && echo ) \
    >> cover.all.md.tmp

hyperfine --warmup 1 --export-markdown cover.md.tmp \
    -n spanr \
    -n jrunlist \
    -n runlist \
    'spanr    cover S288c.rg -o /dev/null' \
    'jrunlist cover S288c.rg -o /dev/null' \
    'runlist  cover S288c.rg -o /dev/null'

cat cover.md.tmp |
    ( cat && echo ) \
    >> cover.all.md.tmp

hyperfine --warmup 1 --export-markdown cover.md.tmp \
    -n spanr \
    -n jrunlist \
    -n runlist \
    'spanr    cover Spom.rg -o /dev/null' \
    'jrunlist cover Spom.rg -o /dev/null' \
    'runlist  cover Spom.rg -o /dev/null'

cat cover.md.tmp |
    ( cat && echo ) \
    >> cover.all.md.tmp

cat cover.all.md.tmp

```

### R7 5800 Windows 11 WSL

| Command    |   Mean [ms] | Min [ms] | Max [ms] |    Relative |
|:-----------|------------:|---------:|---------:|------------:|
| `spanr`    | 107.8 ± 2.6 |    102.6 |    113.5 |        1.00 |
| `jrunlist` | 358.7 ± 9.1 |    346.5 |    374.6 | 3.33 ± 0.12 |
| `runlist`  | 284.7 ± 3.2 |    279.1 |    290.2 | 2.64 ± 0.07 |

| Command    |   Mean [ms] | Min [ms] | Max [ms] |     Relative |
|:-----------|------------:|---------:|---------:|-------------:|
| `spanr`    |  12.9 ± 0.5 |     12.2 |     19.0 |         1.00 |
| `jrunlist` | 347.1 ± 4.2 |    341.5 |    354.7 | 26.84 ± 1.13 |
| `runlist`  | 277.2 ± 4.2 |    272.4 |    285.0 | 21.43 ± 0.92 |

| Command    |    Mean [ms] | Min [ms] | Max [ms] |     Relative |
|:-----------|-------------:|---------:|---------:|-------------:|
| `spanr`    |   21.6 ± 0.8 |     20.2 |     24.7 |         1.00 |
| `jrunlist` | 725.4 ± 16.6 |    704.0 |    754.1 | 33.60 ± 1.45 |
| `runlist`  |  459.5 ± 7.5 |    447.4 |    471.4 | 21.28 ± 0.85 |

### i5-12500H Windows 11 WSL

| Command    |    Mean [ms] | Min [ms] | Max [ms] |    Relative |
|:-----------|-------------:|---------:|---------:|------------:|
| `spanr`    | 132.8 ± 24.2 |    114.0 |    211.3 |        1.00 |
| `jrunlist` | 463.7 ± 13.9 |    445.0 |    491.0 | 3.49 ± 0.64 |
| `runlist`  | 338.2 ± 45.8 |    285.1 |    443.4 | 2.55 ± 0.58 |

| Command    |    Mean [ms] | Min [ms] | Max [ms] |     Relative |
|:-----------|-------------:|---------:|---------:|-------------:|
| `spanr`    |   23.5 ± 7.3 |     13.2 |     41.3 |         1.00 |
| `jrunlist` | 491.8 ± 22.1 |    463.3 |    521.0 | 20.96 ± 6.63 |
| `runlist`  |  287.9 ± 9.7 |    277.8 |    312.1 | 12.27 ± 3.86 |

| Command    |      Mean [ms] | Min [ms] | Max [ms] |     Relative |
|:-----------|---------------:|---------:|---------:|-------------:|
| `spanr`    |     31.2 ± 6.2 |     24.0 |     47.0 |         1.00 |
| `jrunlist` | 1005.1 ± 103.7 |    904.9 |   1177.2 | 32.26 ± 7.27 |
| `runlist`  |   465.5 ± 12.7 |    444.8 |    488.7 | 14.94 ± 3.02 |

### Apple M2 macOS 13.4

| Command    |   Mean [ms] | Min [ms] | Max [ms] |    Relative |
|:-----------|------------:|---------:|---------:|------------:|
| `spanr`    |  51.7 ± 0.9 |     51.3 |     57.5 |        1.00 |
| `jrunlist` | 309.6 ± 5.3 |    296.6 |    316.4 | 5.99 ± 0.15 |
| `runlist`  | 189.6 ± 1.6 |    188.3 |    193.7 | 3.67 ± 0.07 |

| Command    |   Mean [ms] | Min [ms] | Max [ms] |     Relative |
|:-----------|------------:|---------:|---------:|-------------:|
| `spanr`    |  11.8 ± 0.3 |     11.6 |     15.0 |         1.00 |
| `jrunlist` | 309.5 ± 6.8 |    295.6 |    321.7 | 26.28 ± 0.85 |
| `runlist`  | 201.1 ± 1.4 |    199.7 |    204.2 | 17.07 ± 0.43 |

| Command    |    Mean [ms] | Min [ms] | Max [ms] |     Relative |
|:-----------|-------------:|---------:|---------:|-------------:|
| `spanr`    |   19.8 ± 0.6 |     19.5 |     24.1 |         1.00 |
| `jrunlist` | 542.2 ± 11.3 |    529.6 |    556.6 | 27.37 ± 0.95 |
| `runlist`  |  339.0 ± 4.0 |    334.0 |    346.0 | 17.11 ± 0.52 |

## `spanr coverage`

```shell
cd ~/data/intspan

rm coverage.all.md.tmp

hyperfine --warmup 1 --export-markdown coverage.md.tmp \
    -n spanr \
    -n jrunlist \
    -n runlist \
    'spanr    coverage S288c.rg -o /dev/null' \
    'jrunlist cover    S288c.rg -o /dev/null' \
    'runlist  coverage S288c.rg -s S288c.chr.sizes -o /dev/null'

cat coverage.md.tmp |
    ( cat && echo ) \
    >> coverage.all.md.tmp

hyperfine --warmup 1 --export-markdown coverage.md.tmp \
    -n spanr \
    -n jrunlist \
    -n runlist \
    'spanr    coverage Spom.rg -o /dev/null' \
    'jrunlist cover    Spom.rg -o /dev/null' \
    'runlist  coverage Spom.rg -s Spom.chr.sizes -o /dev/null'

cat coverage.md.tmp |
    ( cat && echo ) \
    >> coverage.all.md.tmp

cat coverage.all.md.tmp

```

### R7 5800 Windows 11 WSL

| Command    |    Mean [ms] | Min [ms] | Max [ms] |     Relative |
|:-----------|-------------:|---------:|---------:|-------------:|
| `spanr`    |   18.5 ± 0.8 |     17.3 |     22.9 |         1.00 |
| `jrunlist` | 352.5 ± 12.4 |    341.6 |    383.9 | 19.10 ± 1.06 |
| `runlist`  | 1170.3 ± 8.5 |   1160.2 |   1187.0 | 63.42 ± 2.75 |

| Command    |     Mean [ms] | Min [ms] | Max [ms] |      Relative |
|:-----------|--------------:|---------:|---------:|--------------:|
| `spanr`    |    37.1 ± 0.6 |     35.8 |     38.4 |          1.00 |
| `jrunlist` |  726.4 ± 15.4 |    708.8 |    749.0 |  19.58 ± 0.51 |
| `runlist`  | 6728.6 ± 42.0 |   6667.7 |   6802.6 | 181.37 ± 2.93 |

### i5-12500H Windows 11 WSL

| Command    |     Mean [ms] | Min [ms] | Max [ms] |     Relative |
|:-----------|--------------:|---------:|---------:|-------------:|
| `spanr`    |    28.1 ± 5.3 |     19.3 |     40.6 |         1.00 |
| `jrunlist` |  481.6 ± 17.4 |    451.6 |    504.4 | 17.13 ± 3.29 |
| `runlist`  | 1063.2 ± 31.0 |   1033.3 |   1134.9 | 37.83 ± 7.21 |

| Command    |     Mean [ms] | Min [ms] | Max [ms] |       Relative |
|:-----------|--------------:|---------:|---------:|---------------:|
| `spanr`    |    54.8 ± 9.2 |     43.3 |     90.2 |           1.00 |
| `jrunlist` |  940.0 ± 56.3 |    861.9 |   1030.3 |   17.15 ± 3.05 |
| `runlist`  | 7941.8 ± 79.6 |   7785.5 |   8078.5 | 144.92 ± 24.31 |

### Apple M2 macOS 13.4

| Command    |   Mean [ms] | Min [ms] | Max [ms] |     Relative |
|:-----------|------------:|---------:|---------:|-------------:|
| `spanr`    |  17.5 ± 0.4 |     17.3 |     20.6 |         1.00 |
| `jrunlist` | 307.7 ± 6.0 |    294.4 |    318.3 | 17.54 ± 0.52 |
| `runlist`  | 874.6 ± 6.5 |    867.0 |    884.8 | 49.86 ± 1.18 |

| Command    |     Mean [ms] | Min [ms] | Max [ms] |      Relative |
|:-----------|--------------:|---------:|---------:|--------------:|
| `spanr`    |    37.2 ± 0.7 |     36.9 |     41.8 |          1.00 |
| `jrunlist` |  541.6 ± 10.4 |    521.0 |    553.4 |  14.56 ± 0.39 |
| `runlist`  | 4644.1 ± 10.2 |   4628.3 |   4666.6 | 124.87 ± 2.30 |
