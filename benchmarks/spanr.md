# `spanr`, `jrunlist` and `rangeops`

```shell
brew install intspan
brew install jrunlist
cpanm App::RL

brew install hyperfine

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
        Command being timed: "jrunlist statop chr.sizes sep-gene.yml paralog.yml --op intersect --all -o stdout"
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
        Command being timed: "spanr statop chr.sizes sep-gene.yml paralog.yml --op intersect --all -o stdout"
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
        Command being timed: "runlist stat2 -s chr.sizes sep-gene.yml paralog.yml --op intersect --all --mk -o stdout"
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
s
```

### E5-2690 v3 Ubuntu 14.04 openJDK@8

```text
==> jrunlist
	Command being timed: "jrunlist statop chr.sizes sep-gene.yml paralog.yml --op intersect --all -o stdout"
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
	Command being timed: "spanr statop chr.sizes sep-gene.yml paralog.yml --op intersect --all -o stdout"
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
	Command being timed: "runlist stat2 -s chr.sizes sep-gene.yml paralog.yml --op intersect --all --mk -o stdout"
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
mkdir -p ~/data

curl -o ~/data/S288c.fa.gz \
    -L http://ftp.ensemblgenomes.org/pub/fungi/release-53/fasta/saccharomyces_cerevisiae/dna/Saccharomyces_cerevisiae.R64-1-1.dna_sm.toplevel.fa.gz

curl -o ~/data/Spom.fa.gz \
    -L http://ftp.ensemblgenomes.org/pub/fungi/release-53/fasta/schizosaccharomyces_pombe/dna/Schizosaccharomyces_pombe.ASM294v2.dna_sm.toplevel.fa.gz

faops masked ~/data/S288c.fa.gz > ~/data/S288c.rg
faops masked ~/data/Spom.fa.gz > ~/data/Spom.rg

faops size ~/data/S288c.fa.gz > ~/data/S288c.chr.sizes
faops size ~/data/Spom.fa.gz > ~/data/Spom.chr.sizes

```

```shell
hyperfine --warmup 1 --export-markdown cover.md.tmp \
    -n spanr \
    -n jrunlist \
    -n runlist \
    'faops masked ~/data/S288c.fa.gz | spanr    cover stdin -o /dev/null' \
    'faops masked ~/data/S288c.fa.gz | jrunlist cover stdin -o /dev/null' \
    'faops masked ~/data/S288c.fa.gz | runlist  cover stdin -o /dev/null'

cat cover.md.tmp

hyperfine --warmup 1 --export-markdown cover.md.tmp \
    -n spanr \
    -n jrunlist \
    -n runlist \
    'spanr    cover ~/data/S288c.rg -o /dev/null' \
    'jrunlist cover ~/data/S288c.rg -o /dev/null' \
    'runlist  cover ~/data/S288c.rg -o /dev/null'

cat cover.md.tmp

hyperfine --warmup 1 --export-markdown cover.md.tmp \
    -n spanr \
    -n jrunlist \
    -n runlist \
    'spanr    cover ~/data/Spom.rg -o /dev/null' \
    'jrunlist cover ~/data/Spom.rg -o /dev/null' \
    'runlist  cover ~/data/Spom.rg -o /dev/null'

cat cover.md.tmp

```

### R7 5800 Windows 11 WSL

| Command    |   Mean [ms] | Min [ms] | Max [ms] |    Relative |
|:-----------|------------:|---------:|---------:|------------:|
| `spanr`    | 103.5 ± 1.6 |     99.1 |    106.1 |        1.00 |
| `jrunlist` | 357.4 ± 5.4 |    347.9 |    365.6 | 3.45 ± 0.08 |
| `runlist`  | 290.6 ± 3.1 |    285.4 |    295.8 | 2.81 ± 0.05 |

| Command    |   Mean [ms] | Min [ms] | Max [ms] |     Relative |
|:-----------|------------:|---------:|---------:|-------------:|
| `spanr`    |  12.5 ± 0.2 |     11.8 |     13.4 |         1.00 |
| `jrunlist` | 368.1 ± 9.2 |    358.5 |    381.7 | 29.43 ± 0.94 |
| `runlist`  | 287.3 ± 3.9 |    280.9 |    293.1 | 22.97 ± 0.55 |

| Command    |    Mean [ms] | Min [ms] | Max [ms] |     Relative |
|:-----------|-------------:|---------:|---------:|-------------:|
| `spanr`    |   20.4 ± 0.4 |     19.5 |     22.1 |         1.00 |
| `jrunlist` | 754.8 ± 24.8 |    725.3 |    798.9 | 37.09 ± 1.39 |
| `runlist`  |  470.4 ± 8.2 |    459.7 |    484.4 | 23.11 ± 0.58 |

### R5 4600U Windows 11 WSL

| Command    |    Mean [ms] | Min [ms] | Max [ms] |    Relative |
|:-----------|-------------:|---------:|---------:|------------:|
| `spanr`    |   93.2 ± 4.3 |     89.3 |    107.8 |        1.00 |
| `jrunlist` | 574.4 ± 29.8 |    530.8 |    631.4 | 6.16 ± 0.43 |
| `runlist`  |  408.1 ± 9.2 |    395.6 |    424.6 | 4.38 ± 0.22 |

| Command    |    Mean [ms] | Min [ms] | Max [ms] |     Relative |
|:-----------|-------------:|---------:|---------:|-------------:|
| `spanr`    |   16.1 ± 0.8 |     14.7 |     19.4 |         1.00 |
| `jrunlist` | 551.9 ± 34.0 |    515.0 |    602.9 | 34.33 ± 2.74 |
| `runlist`  | 431.8 ± 11.7 |    413.2 |    451.6 | 26.86 ± 1.54 |

| Command    |     Mean [ms] | Min [ms] | Max [ms] |     Relative |
|:-----------|--------------:|---------:|---------:|-------------:|
| `spanr`    |    25.0 ± 1.1 |     23.5 |     30.2 |         1.00 |
| `jrunlist` | 1203.8 ± 39.3 |   1162.0 |   1271.6 | 48.23 ± 2.69 |
| `runlist`  |  754.5 ± 54.9 |    693.0 |    857.6 | 30.23 ± 2.59 |

## `spanr coverage`

```shell
hyperfine --warmup 1 --export-markdown coverage.md.tmp \
    -n spanr \
    -n jrunlist \
    -n runlist \
    'spanr    coverage ~/data/S288c.rg -o /dev/null' \
    'jrunlist cover    ~/data/S288c.rg -o /dev/null' \
    'runlist  coverage ~/data/S288c.rg -s ~/data/S288c.chr.sizes -o /dev/null'

cat coverage.md.tmp

hyperfine --warmup 1 --export-markdown coverage.md.tmp \
    -n spanr \
    -n jrunlist \
    -n runlist \
    'spanr    coverage ~/data/Spom.rg -o /dev/null' \
    'jrunlist cover    ~/data/Spom.rg -o /dev/null' \
    'runlist  coverage ~/data/Spom.rg -s ~/data/Spom.chr.sizes -o /dev/null'

cat coverage.md.tmp

```

### R7 5800 Windows 11 WSL

| Command    |    Mean [ms] | Min [ms] | Max [ms] |     Relative |
|:-----------|-------------:|---------:|---------:|-------------:|
| `spanr`    |   18.1 ± 0.4 |     17.3 |     20.7 |         1.00 |
| `jrunlist` |  361.1 ± 4.1 |    350.9 |    366.0 | 19.96 ± 0.47 |
| `runlist`  | 1204.7 ± 8.9 |   1190.8 |   1224.3 | 66.59 ± 1.44 |

| Command    |     Mean [ms] | Min [ms] | Max [ms] |      Relative |
|:-----------|--------------:|---------:|---------:|--------------:|
| `spanr`    |    36.6 ± 0.4 |     35.5 |     37.4 |          1.00 |
| `jrunlist` |  786.5 ± 60.8 |    740.2 |    907.5 |  21.50 ± 1.68 |
| `runlist`  | 6826.4 ± 54.5 |   6748.5 |   6891.2 | 186.59 ± 2.55 |

### R5 4600U Windows 11 WSL

| Command    |      Mean [ms] | Min [ms] | Max [ms] |     Relative |
|:-----------|---------------:|---------:|---------:|-------------:|
| `spanr`    |     23.8 ± 1.0 |     22.0 |     27.5 |         1.00 |
| `jrunlist` |   562.1 ± 33.2 |    523.1 |    626.3 | 23.58 ± 1.71 |
| `runlist`  | 1862.1 ± 122.8 |   1769.0 |   2157.5 | 78.13 ± 6.13 |

| Command    |       Mean [ms] | Min [ms] | Max [ms] |       Relative |
|:-----------|----------------:|---------:|---------:|---------------:|
| `spanr`    |      47.6 ± 1.1 |     45.7 |     50.3 |           1.00 |
| `jrunlist` |   1141.6 ± 32.0 |   1111.4 |   1205.6 |   23.98 ± 0.86 |
| `runlist`  | 10334.2 ± 755.4 |   9766.4 |  12291.9 | 217.11 ± 16.60 |
