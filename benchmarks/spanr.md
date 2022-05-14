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

* OSX 10.14 i7-8700k oracleJDK8

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

* Ubuntu 14.04 E5-2690 v3 openJDK@18

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
    'faops masked ~/data/S288c.fa.gz | spanr    cover stdin -o /dev/null' \
    -n jrunlist \
    'faops masked ~/data/S288c.fa.gz | jrunlist cover stdin -o /dev/null' \
    -n runlist \
    'faops masked ~/data/S288c.fa.gz | runlist  cover stdin -o /dev/null'

cat cover.md.tmp

hyperfine --warmup 1 --export-markdown cover.md.tmp \
    -n spanr \
    'spanr    cover ~/data/S288c.rg -o /dev/null' \
    -n jrunlist \
    'jrunlist cover ~/data/S288c.rg -o /dev/null' \
    -n runlist \
    'runlist  cover ~/data/S288c.rg -o /dev/null'

cat cover.md.tmp

hyperfine --warmup 1 --export-markdown cover.md.tmp \
    -n spanr \
    'spanr    cover ~/data/Spom.rg -o /dev/null' \
    -n jrunlist \
    'jrunlist cover ~/data/Spom.rg -o /dev/null' \
    -n runlist \
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

## `spanr coverage`

```shell
hyperfine --warmup 1 --export-markdown coverage.md.tmp \
    -n spanr \
    'spanr    coverage ~/data/S288c.rg -o /dev/null' \
    -n jrunlist \
    'jrunlist cover    ~/data/S288c.rg -o /dev/null' \
    -n runlist \
    'runlist  coverage ~/data/S288c.rg -s ~/data/S288c.chr.sizes -o /dev/null'

cat coverage.md.tmp

hyperfine --warmup 1 --export-markdown coverage.md.tmp \
    -n spanr \
    'spanr    coverage ~/data/Spom.rg -o /dev/null' \
    -n jrunlist \
    'jrunlist cover    ~/data/Spom.rg -o /dev/null' \
    -n runlist \
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
