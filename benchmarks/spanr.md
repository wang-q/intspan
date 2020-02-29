# `spanr`, `jrunlist` and `rangeops`

```bash
brew install intspan
brew install jrunlist
cpanm App::RL

brew install hyperfine

```

## `bash benchmarks/spanr/run.sh`

```bash
bash ~/Scripts/rust/intspan/benchmarks/spanr/run.sh

rm ~/Scripts/rust/intspan/benchmarks/spanr/*.tmp

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

```bash
hyperfine --warmup 1 --export-markdown cover.md.tmp \
    'faops masked ~/data/alignment/Ensembl/S288c/*.fa | spanr    cover stdin -o /dev/null' \
    'faops masked ~/data/alignment/Ensembl/S288c/*.fa | jrunlist cover stdin -o /dev/null' \
    'faops masked ~/data/alignment/Ensembl/S288c/*.fa | runlist  cover stdin -o /dev/null'

hyperfine --warmup 1 --export-markdown cover.md.tmp \
    'faops masked ~/data/alignment/Ensembl/Spom/*.fa | spanr    cover stdin -o /dev/null' \
    'faops masked ~/data/alignment/Ensembl/Spom/*.fa | jrunlist cover stdin -o /dev/null' \
    'faops masked ~/data/alignment/Ensembl/Spom/*.fa | runlist  cover stdin -o /dev/null'

```

| Command  |   Mean [ms] | Min [ms] | Max [ms] |    Relative |
|:---------|------------:|---------:|---------:|------------:|
| spanr    | 186.0 ± 0.9 |    184.5 |    187.5 |        1.00 |
| jrunlist | 466.4 ± 2.4 |    462.8 |    470.0 | 2.51 ± 0.02 |
| runlist  | 379.1 ± 1.8 |    376.5 |    382.2 | 2.04 ± 0.01 |

| Command  |       Mean [s] | Min [s] | Max [s] |     Relative |
|:---------|---------------:|--------:|--------:|-------------:|
| spanr    | 21.592 ± 0.150 |  21.407 |  21.862 | 35.54 ± 0.82 |
| jrunlist |  1.524 ± 0.022 |   1.509 |   1.577 |  2.51 ± 0.07 |
| runlist  |  0.608 ± 0.013 |   0.594 |   0.635 |         1.00 |

