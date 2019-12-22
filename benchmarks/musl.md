# `gcc` vs `musl`

* Ubuntu 14.04 E5-2690 v3
* rustc 1.40
* gcc with lto
* musl with lto

## `bash benchmarks/musl/run.sh`

```bash
# cargo install --path ~/Scripts/rust/intspan --force

bash ~/Scripts/rust/intspan/benchmarks/musl/run.sh

find ~/Scripts/rust/intspan/benchmarks/musl/* |
    grep -v "run.sh" |
    xargs rm -fr

```

## Results

* sort

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| cargo | 127.9 ± 22.7 | 97.3 | 147.7 | 1.2 |
| gcc | 107.9 ± 21.6 | 92.2 | 142.5 | 1.0 |
| musl | 132.9 ± 23.2 | 102.2 | 151.4 | 1.2 |

* clean

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| gcc | 3.811 ± 0.012 | 3.791 | 3.827 | 1.0 |
| musl | 4.898 ± 0.063 | 4.784 | 4.985 | 1.3 |

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| cargo | 4.266 ± 0.075 | 4.224 | 4.478 | 1.0 |
| gcc | 6.090 ± 2.789 | 3.824 | 9.361 | 1.4 |
| musl | 7.869 ± 3.102 | 4.839 | 11.355 | 1.8 |

* merge

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| cargo | 2.991 ± 0.006 | 2.976 | 2.999 | 1.1 |
| gcc | 2.712 ± 0.003 | 2.707 | 2.716 | 1.0 |
| musl | 4.527 ± 0.086 | 4.492 | 4.770 | 1.7 |

* clean2

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| cargo | 5.152 ± 0.026 | 5.132 | 5.221 | 1.1 |
| gcc | 4.821 ± 0.003 | 4.817 | 4.826 | 1.0 |
| musl | 5.983 ± 0.924 | 3.370 | 6.544 | 1.2 |

* connect

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| cargo | 387.0 ± 0.7 | 386.2 | 387.8 | 1.0 |
| gcc | 374.5 ± 74.3 | 247.8 | 532.1 | 1.0 |
| musl | 383.0 ± 0.6 | 382.0 | 384.2 | 1.0 |

* filter

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| cargo | 59.2 ± 1.1 | 58.4 | 66.2 | 1.1 |
| gcc | 54.7 ± 0.3 | 54.2 | 55.7 | 1.0 |
| musl | 55.6 ± 22.3 | 30.9 | 126.5 | 1.0 |
