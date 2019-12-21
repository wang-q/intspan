# `gcc` vs `musl`

## `bash benchmarks/musl/run.sh`

```bash
bash ~/Scripts/rust/intspan/benchmarks/musl/run.sh

find ~/Scripts/rust/intspan/benchmarks/musl/* |
    grep -v "run.sh" |
    xargs rm -fr

```

* Ubuntu 14.04 E5-2690 v3

## Results

* sort

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| gcc | 173.2 ± 78.5 | 77.2 | 281.1 | 1.0 |
| musl | 193.7 ± 80.9 | 102.8 | 295.5 | 1.1 |

* clean

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| gcc | 3.811 ± 0.012 | 3.791 | 3.827 | 1.0 |
| musl | 4.898 ± 0.063 | 4.784 | 4.985 | 1.3 |

* merge

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| gcc | 6.684 ± 1.400 | 3.795 | 7.544 | 1.0 |
| musl | 9.993 ± 0.001 | 9.991 | 9.994 | 1.5 |

* clean2

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| gcc | 2.642 ± 1.149 | 2.061 | 4.818 | 1.0 |
| musl | 2.711 ± 0.012 | 2.691 | 2.727 | 1.0 |

* connect

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| gcc | 159.6 ± 21.7 | 146.7 | 198.0 | 1.0 |
| musl | 158.1 ± 0.7 | 157.5 | 160.2 | 1.0 |

* filter

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| gcc | 25.4 ± 4.1 | 21.0 | 39.5 | 1.1 |
| musl | 23.2 ± 0.3 | 22.8 | 24.2 | 1.0 |
