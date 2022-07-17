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
rm cover.all.md.tmp

hyperfine --warmup 1 --export-markdown cover.md.tmp \
    -n spanr \
    -n jrunlist \
    -n runlist \
    'faops masked ~/data/S288c.fa.gz | spanr    cover stdin -o /dev/null' \
    'faops masked ~/data/S288c.fa.gz | jrunlist cover stdin -o /dev/null' \
    'faops masked ~/data/S288c.fa.gz | runlist  cover stdin -o /dev/null'

cat cover.md.tmp |
    ( cat && echo ) \
    >> cover.all.md.tmp

hyperfine --warmup 1 --export-markdown cover.md.tmp \
    -n spanr \
    -n jrunlist \
    -n runlist \
    'spanr    cover ~/data/S288c.rg -o /dev/null' \
    'jrunlist cover ~/data/S288c.rg -o /dev/null' \
    'runlist  cover ~/data/S288c.rg -o /dev/null'

cat cover.md.tmp |
    ( cat && echo ) \
    >> cover.all.md.tmp

hyperfine --warmup 1 --export-markdown cover.md.tmp \
    -n spanr \
    -n jrunlist \
    -n runlist \
    'spanr    cover ~/data/Spom.rg -o /dev/null' \
    'jrunlist cover ~/data/Spom.rg -o /dev/null' \
    'runlist  cover ~/data/Spom.rg -o /dev/null'

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
| `spanr`    |  103.9 ± 9.0 |     97.2 |    144.1 |        1.00 |
| `jrunlist` | 424.3 ± 12.6 |    410.4 |    447.2 | 4.08 ± 0.37 |
| `runlist`  | 312.0 ± 35.3 |    270.7 |    367.9 | 3.00 ± 0.43 |

| Command    |    Mean [ms] | Min [ms] | Max [ms] |     Relative |
|:-----------|-------------:|---------:|---------:|-------------:|
| `spanr`    |   23.2 ± 4.1 |     14.4 |     37.5 |         1.00 |
| `jrunlist` |  417.5 ± 9.6 |    399.7 |    432.4 | 18.03 ± 3.20 |
| `runlist`  | 279.7 ± 22.8 |    262.5 |    323.2 | 12.08 ± 2.35 |

| Command    |    Mean [ms] | Min [ms] | Max [ms] |     Relative |
|:-----------|-------------:|---------:|---------:|-------------:|
| `spanr`    |   31.7 ± 8.1 |     21.6 |     45.2 |         1.00 |
| `jrunlist` | 897.2 ± 91.8 |    820.8 |   1140.6 | 28.34 ± 7.81 |
| `runlist`  | 473.0 ± 37.5 |    431.7 |    532.6 | 14.94 ± 4.00 |

### i5-12500H Windows 11 Multipass Ubuntu

| Command    |    Mean [ms] | Min [ms] | Max [ms] |    Relative |
|:-----------|-------------:|---------:|---------:|------------:|
| `spanr`    |   79.7 ± 2.3 |     76.2 |     89.3 |        1.00 |
| `jrunlist` | 758.7 ± 19.9 |    726.1 |    790.1 | 9.52 ± 0.37 |
| `runlist`  |  281.4 ± 6.8 |    274.7 |    295.6 | 3.53 ± 0.13 |

| Command    |    Mean [ms] | Min [ms] | Max [ms] |     Relative |
|:-----------|-------------:|---------:|---------:|-------------:|
| `spanr`    |    9.9 ± 0.8 |      8.9 |     16.2 |         1.00 |
| `jrunlist` | 681.3 ± 35.8 |    641.2 |    769.2 | 69.06 ± 6.92 |
| `runlist`  |  269.4 ± 9.4 |    260.3 |    294.4 | 27.31 ± 2.52 |

| Command    |    Mean [ms] | Min [ms] | Max [ms] |     Relative |
|:-----------|-------------:|---------:|---------:|-------------:|
| `spanr`    |   15.5 ± 0.4 |     14.5 |     17.0 |         1.00 |
| `jrunlist` | 1047.5 ± 9.7 |   1035.3 |   1058.7 | 67.53 ± 1.96 |
| `runlist`  |  421.5 ± 2.0 |    416.5 |    423.8 | 27.17 ± 0.76 |

### i7-8700K Ubuntu 22.04

| Command    |    Mean [ms] | Min [ms] | Max [ms] |    Relative |
|:-----------|-------------:|---------:|---------:|------------:|
| `spanr`    |   83.1 ± 4.0 |     80.1 |    103.5 |        1.00 |
| `jrunlist` | 501.7 ± 14.6 |    471.5 |    517.9 | 6.03 ± 0.34 |
| `runlist`  |  355.0 ± 6.3 |    348.0 |    371.6 | 4.27 ± 0.22 |

| Command    |    Mean [ms] | Min [ms] | Max [ms] |     Relative |
|:-----------|-------------:|---------:|---------:|-------------:|
| `spanr`    |   13.6 ± 0.5 |     13.0 |     16.6 |         1.00 |
| `jrunlist` | 464.0 ± 19.7 |    437.8 |    491.0 | 34.15 ± 1.97 |
| `runlist`  | 389.1 ± 25.0 |    365.7 |    455.6 | 28.64 ± 2.15 |

| Command    |     Mean [ms] | Min [ms] | Max [ms] |     Relative |
|:-----------|--------------:|---------:|---------:|-------------:|
| `spanr`    |    22.0 ± 0.6 |     21.0 |     24.7 |         1.00 |
| `jrunlist` | 1023.2 ± 75.0 |    941.3 |   1174.9 | 46.61 ± 3.67 |
| `runlist`  |   620.3 ± 8.4 |    600.0 |    630.9 | 28.26 ± 0.89 |

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

### i7-4770HQ macOS Big Sur

| Command    |     Mean [ms] | Min [ms] | Max [ms] |    Relative |
|:-----------|--------------:|---------:|---------:|------------:|
| `spanr`    |   115.4 ± 7.9 |    110.2 |    134.2 |        1.00 |
| `jrunlist` |  793.0 ± 20.8 |    777.8 |    849.0 | 6.87 ± 0.50 |
| `runlist`  | 782.4 ± 188.1 |    618.1 |   1081.1 | 6.78 ± 1.69 |

| Command    |    Mean [ms] | Min [ms] | Max [ms] |     Relative |
|:-----------|-------------:|---------:|---------:|-------------:|
| `spanr`    |   51.4 ± 9.8 |     39.9 |     71.6 |         1.00 |
| `jrunlist` | 822.4 ± 22.7 |    793.2 |    853.3 | 15.99 ± 3.07 |
| `runlist`  | 677.9 ± 21.1 |    652.3 |    715.5 | 13.18 ± 2.54 |

| Command    |      Mean [ms] | Min [ms] | Max [ms] |     Relative |
|:-----------|---------------:|---------:|---------:|-------------:|
| `spanr`    |     77.2 ± 7.8 |     67.7 |     96.4 |         1.00 |
| `jrunlist` | 2001.5 ± 144.2 |   1833.7 |   2287.1 | 25.92 ± 3.22 |
| `runlist`  |  1199.2 ± 88.3 |   1103.1 |   1316.8 | 15.53 ± 1.94 |

## `spanr coverage`

```shell
rm coverage.all.md.tmp

hyperfine --warmup 1 --export-markdown coverage.md.tmp \
    -n spanr \
    -n jrunlist \
    -n runlist \
    'spanr    coverage ~/data/S288c.rg -o /dev/null' \
    'jrunlist cover    ~/data/S288c.rg -o /dev/null' \
    'runlist  coverage ~/data/S288c.rg -s ~/data/S288c.chr.sizes -o /dev/null'

cat coverage.md.tmp |
    ( cat && echo ) \
    >> coverage.all.md.tmp

hyperfine --warmup 1 --export-markdown coverage.md.tmp \
    -n spanr \
    -n jrunlist \
    -n runlist \
    'spanr    coverage ~/data/Spom.rg -o /dev/null' \
    'jrunlist cover    ~/data/Spom.rg -o /dev/null' \
    'runlist  coverage ~/data/Spom.rg -s ~/data/Spom.chr.sizes -o /dev/null'

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
| `spanr`    |    27.5 ± 3.4 |     21.1 |     48.5 |         1.00 |
| `jrunlist` |  400.5 ± 17.3 |    377.8 |    438.3 | 14.56 ± 1.89 |
| `runlist`  | 1007.6 ± 38.1 |    967.6 |   1066.7 | 36.62 ± 4.68 |

| Command    |      Mean [ms] | Min [ms] | Max [ms] |       Relative |
|:-----------|---------------:|---------:|---------:|---------------:|
| `spanr`    |     53.6 ± 4.8 |     44.0 |     64.8 |           1.00 |
| `jrunlist` |   875.3 ± 62.1 |    801.4 |   1011.6 |   16.34 ± 1.86 |
| `runlist`  | 6994.4 ± 172.9 |   6682.3 |   7210.3 | 130.60 ± 12.09 |

### i5-12500H Windows 11 Multipass Ubuntu

| Command    |     Mean [ms] | Min [ms] | Max [ms] |     Relative |
|:-----------|--------------:|---------:|---------:|-------------:|
| `spanr`    |    15.5 ± 0.7 |     14.4 |     21.6 |         1.00 |
| `jrunlist` |  661.8 ± 12.6 |    644.7 |    685.0 | 42.57 ± 2.10 |
| `runlist`  | 1020.2 ± 10.9 |   1008.9 |   1043.1 | 65.63 ± 3.07 |

| Command    |     Mean [ms] | Min [ms] | Max [ms] |      Relative |
|:-----------|--------------:|---------:|---------:|--------------:|
| `spanr`    |    34.6 ± 0.8 |     33.0 |     37.5 |          1.00 |
| `jrunlist` |  1039.7 ± 7.4 |   1030.8 |   1054.4 |  30.05 ± 0.70 |
| `runlist`  | 7079.5 ± 51.8 |   6991.1 |   7152.3 | 204.64 ± 4.75 |

### i7-8700K Ubuntu 22.04

| Command    |     Mean [ms] | Min [ms] | Max [ms] |     Relative |
|:-----------|--------------:|---------:|---------:|-------------:|
| `spanr`    |    21.2 ± 1.6 |     19.9 |     27.6 |         1.00 |
| `jrunlist` |  494.6 ± 20.9 |    465.7 |    536.0 | 23.29 ± 2.06 |
| `runlist`  | 1602.7 ± 87.5 |   1529.9 |   1784.0 | 75.47 ± 7.16 |

| Command    |      Mean [ms] | Min [ms] | Max [ms] |      Relative |
|:-----------|---------------:|---------:|---------:|--------------:|
| `spanr`    |     45.2 ± 1.5 |     44.0 |     53.7 |          1.00 |
| `jrunlist` |  1031.7 ± 40.1 |    996.5 |   1118.2 |  22.82 ± 1.16 |
| `runlist`  | 9033.2 ± 234.4 |   8682.3 |   9528.2 | 199.81 ± 8.35 |

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

### i7-4770HQ macOS Big Sur

| Command    |      Mean [ms] | Min [ms] | Max [ms] |     Relative |
|:-----------|---------------:|---------:|---------:|-------------:|
| `spanr`    |    60.0 ± 11.2 |     49.5 |     88.2 |         1.00 |
| `jrunlist` |  983.1 ± 142.7 |    792.9 |   1173.6 | 16.38 ± 3.87 |
| `runlist`  | 2517.8 ± 122.4 |   2422.0 |   2840.9 | 41.94 ± 8.08 |

| Command    |       Mean [ms] | Min [ms] | Max [ms] |      Relative |
|:-----------|----------------:|---------:|---------:|--------------:|
| `spanr`    |     101.8 ± 2.6 |     98.5 |    112.8 |          1.00 |
| `jrunlist` |  1759.1 ± 251.1 |   1569.7 |   2348.3 |  17.27 ± 2.50 |
| `runlist`  | 14284.2 ± 597.6 |  13734.9 |  15798.3 | 140.26 ± 6.88 |
