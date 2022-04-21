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

* Ubuntu 14.04 E5-2690 v3 openJDK9

```text
==> jrunlist
        Command being timed: "jrunlist statop chr.sizes sep-gene.yml paralog.yml --op intersect --all -o stdout"
        User time (seconds): 14.33
        System time (seconds): 1.03
        Percent of CPU this job got: 406%
        Elapsed (wall clock) time (h:mm:ss or m:ss): 0:03.78
        Average shared text size (kbytes): 0
        Average unshared data size (kbytes): 0
        Average stack size (kbytes): 0
        Average total size (kbytes): 0
        Maximum resident set size (kbytes): 1649228
        Average resident set size (kbytes): 0
        Major (requiring I/O) page faults: 0
        Minor (reclaiming a frame) page faults: 104173
        Voluntary context switches: 12854
        Involuntary context switches: 1149
        Swaps: 0
        File system inputs: 0
        File system outputs: 3552
        Socket messages sent: 0
        Socket messages received: 0
        Signals delivered: 0
        Page size (bytes): 4096
        Exit status: 0
==> spanr
        Command being timed: "spanr statop chr.sizes sep-gene.yml paralog.yml --op intersect --all -o stdout"
        User time (seconds): 5.31
        System time (seconds): 0.07
        Percent of CPU this job got: 100%
        Elapsed (wall clock) time (h:mm:ss or m:ss): 0:05.38
        Average shared text size (kbytes): 0
        Average unshared data size (kbytes): 0
        Average stack size (kbytes): 0
        Average total size (kbytes): 0
        Maximum resident set size (kbytes): 83524
        Average resident set size (kbytes): 0
        Major (requiring I/O) page faults: 0
        Minor (reclaiming a frame) page faults: 46893
        Voluntary context switches: 1
        Involuntary context switches: 7
        Swaps: 0
        File system inputs: 0
        File system outputs: 3488
        Socket messages sent: 0
        Socket messages received: 0
        Signals delivered: 0
        Page size (bytes): 4096
        Exit status: 0
==> App::RL
        Command being timed: "runlist stat2 -s chr.sizes sep-gene.yml paralog.yml --op intersect --all --mk -o stdout"
        User time (seconds): 281.31
        System time (seconds): 0.12
        Percent of CPU this job got: 100%
        Elapsed (wall clock) time (h:mm:ss or m:ss): 4:41.33
        Average shared text size (kbytes): 0
        Average unshared data size (kbytes): 0
        Average stack size (kbytes): 0
        Average total size (kbytes): 0
        Maximum resident set size (kbytes): 116732
        Average resident set size (kbytes): 0
        Major (requiring I/O) page faults: 0
        Minor (reclaiming a frame) page faults: 134101
        Voluntary context switches: 1
        Involuntary context switches: 1162
        Swaps: 0
        File system inputs: 0
        File system outputs: 3488
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

faops masked ~/data/S288c.fa.gz > ~/data/S288c.ranges
faops masked ~/data/Spom.fa.gz > ~/data/Spom.ranges

faops size ~/data/S288c.fa.gz > ~/data/S288c.chr.sizes
faops size ~/data/Spom.fa.gz > ~/data/Spom.chr.sizes

```

```shell
hyperfine --warmup 1 --export-markdown cover.md.tmp \
    'faops masked ~/data/S288c.fa.gz | spanr    cover stdin -o /dev/null' \
    'faops masked ~/data/S288c.fa.gz | jrunlist cover stdin -o /dev/null' \
    'faops masked ~/data/S288c.fa.gz | runlist  cover stdin -o /dev/null'

hyperfine --warmup 1 --export-markdown cover.md.tmp \
    'spanr    cover ~/data/S288c.ranges -o /dev/null' \
    'jrunlist cover ~/data/S288c.ranges -o /dev/null' \
    'runlist  cover ~/data/S288c.ranges -o /dev/null'

hyperfine --warmup 1 --export-markdown cover.md.tmp \
    'spanr    cover ~/data/Spom.ranges -o /dev/null' \
    'jrunlist cover ~/data/Spom.ranges -o /dev/null' \
    'runlist  cover ~/data/Spom.ranges -o /dev/null'

```

| Command  |    Mean [ms] | Min [ms] | Max [ms] |    Relative |
|:---------|-------------:|---------:|---------:|------------:|
| spanr    |   47.3 ± 1.1 |     45.6 |     51.7 |        1.00 |
| jrunlist | 470.2 ± 13.5 |    457.1 |    505.2 | 9.94 ± 0.36 |
| runlist  |  365.6 ± 2.3 |    362.5 |    370.1 | 7.73 ± 0.18 |

| Command                                           |    Mean [ms] | Min [ms] | Max [ms] |     Relative |
|:--------------------------------------------------|-------------:|---------:|---------:|-------------:|
| `spanr    cover ~/data/S288c.ranges -o /dev/null` |   13.3 ± 0.6 |     12.5 |     16.8 |         1.00 |
| `jrunlist cover ~/data/S288c.ranges -o /dev/null` | 371.8 ± 15.4 |    356.0 |    404.2 | 27.91 ± 1.65 |
| `runlist  cover ~/data/S288c.ranges -o /dev/null` |  284.5 ± 2.7 |    281.4 |    288.8 | 21.35 ± 0.93 |

| Command                                          |    Mean [ms] | Min [ms] | Max [ms] |     Relative |
|:-------------------------------------------------|-------------:|---------:|---------:|-------------:|
| `spanr    cover ~/data/Spom.ranges -o /dev/null` |   20.7 ± 1.3 |     19.2 |     26.9 |         1.00 |
| `jrunlist cover ~/data/Spom.ranges -o /dev/null` | 824.9 ± 51.5 |    764.1 |    896.3 | 39.77 ± 3.56 |
| `runlist  cover ~/data/Spom.ranges -o /dev/null` |  473.9 ± 8.9 |    460.6 |    486.7 | 22.85 ± 1.53 |

## `spanr coverage`

```shell
hyperfine --warmup 1 --export-markdown cover.md.tmp \
    'spanr    coverage ~/data/S288c.ranges -o /dev/null' \
    'jrunlist cover    ~/data/S288c.ranges -o /dev/null' \
    'runlist  coverage ~/data/S288c.ranges -s ~/data/S288c.chr.sizes -o /dev/null'

hyperfine --warmup 1 --export-markdown cover.md.tmp \
    'spanr    coverage ~/data/Spom.ranges -o /dev/null' \
    'jrunlist cover    ~/data/Spom.ranges -o /dev/null' \
    'runlist  coverage ~/data/Spom.ranges -s ~/data/Spom.chr.sizes -o /dev/null'


```

| Command  |     Mean [ms] | Min [ms] | Max [ms] |     Relative |
|:---------|--------------:|---------:|---------:|-------------:|
| spanr    |    18.0 ± 0.9 |     17.2 |     28.1 |         1.00 |
| jrunlist |  365.1 ± 10.2 |    350.4 |    380.2 | 20.33 ± 1.20 |
| runlist  | 1198.0 ± 15.5 |   1176.5 |   1232.3 | 66.69 ± 3.59 |

| Command  |     Mean [ms] | Min [ms] | Max [ms] |      Relative |
|:---------|--------------:|---------:|---------:|--------------:|
| spanr    |    37.6 ± 1.9 |     35.8 |     46.2 |          1.00 |
| jrunlist |  732.9 ± 14.8 |    719.4 |    765.3 |  19.49 ± 1.05 |
| runlist  | 6914.3 ± 29.1 |   6870.1 |   6960.8 | 183.88 ± 9.26 |
