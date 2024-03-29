# `gcc` vs `musl`

* Ubuntu 14.04 E5-2690 v3
    * rustc 1.40
    * gcc with lto
    * musl with lto
* Ryzen 7 5800 Windows 11 WSL
    * rustc 1.60.0
* i5-12500H Windows 11 WSL
    * rustc 1.61.0

## `bash benchmarks/musl/run.sh`

```shell
# cargo install --path ~/Scripts/rust/intspan --force

bash ~/Scripts/intspan/benchmarks/musl/run.sh

find ~/Scripts/intspan/benchmarks/musl/* |
    grep -v "run.sh" |
    grep -v ".gitignore" |
    xargs rm -fr

```

## Results

* Above - E5-2690 v3
* Mid - Ryzen 7 5800
* Below - i5-12500H

* sort

| Command |    Mean [ms] | Min [ms] | Max [ms] | Relative |
|:--------|-------------:|---------:|---------:|---------:|
| cargo   | 127.9 ± 22.7 |     97.3 |    147.7 |      1.2 |
| gcc     | 107.9 ± 21.6 |     92.2 |    142.5 |      1.0 |
| musl    | 132.9 ± 23.2 |    102.2 |    151.4 |      1.2 |

| Command |  Mean [ms] | Min [ms] | Max [ms] |    Relative |
|:--------|-----------:|---------:|---------:|------------:|
| `cargo` | 36.6 ± 0.5 |     35.8 |     38.0 |        1.00 |
| `gcc`   | 68.0 ± 1.3 |     63.3 |     71.4 | 1.86 ± 0.04 |
| `musl`  | 74.6 ± 1.9 |     70.1 |     77.6 | 2.04 ± 0.06 |

| Command |   Mean [ms] | Min [ms] | Max [ms] |    Relative |
|:--------|------------:|---------:|---------:|------------:|
| `cargo` |  39.2 ± 2.2 |     36.9 |     51.5 |        1.00 |
| `gcc`   | 79.6 ± 10.6 |     65.9 |    105.5 | 2.03 ± 0.29 |
| `musl`  |  91.7 ± 8.3 |     78.9 |    114.1 | 2.34 ± 0.25 |

* clean

| Command |      Mean [s] | Min [s] | Max [s] | Relative |
|:--------|--------------:|--------:|--------:|---------:|
| cargo   | 4.266 ± 0.075 |   4.224 |   4.478 |      1.0 |
| gcc     | 6.090 ± 2.789 |   3.824 |   9.361 |      1.4 |
| musl    | 7.869 ± 3.102 |   4.839 |  11.355 |      1.8 |

| Command |      Mean [s] | Min [s] | Max [s] |    Relative |
|:--------|--------------:|--------:|--------:|------------:|
| `cargo` | 1.446 ± 0.344 |   1.279 |   2.351 | 1.06 ± 0.25 |
| `gcc`   | 1.361 ± 0.029 |   1.334 |   1.420 |        1.00 |
| `musl`  | 2.624 ± 0.029 |   2.586 |   2.697 | 1.93 ± 0.05 |

| Command |      Mean [s] | Min [s] | Max [s] |    Relative |
|:--------|--------------:|--------:|--------:|------------:|
| `cargo` | 1.501 ± 0.025 |   1.466 |   1.551 | 1.04 ± 0.02 |
| `gcc`   | 1.440 ± 0.021 |   1.406 |   1.475 |        1.00 |
| `musl`  | 3.942 ± 0.060 |   3.849 |   4.027 | 2.74 ± 0.06 |

* merge

| Command |      Mean [s] | Min [s] | Max [s] | Relative |
|:--------|--------------:|--------:|--------:|---------:|
| cargo   | 2.991 ± 0.006 |   2.976 |   2.999 |      1.1 |
| gcc     | 2.712 ± 0.003 |   2.707 |   2.716 |      1.0 |
| musl    | 4.527 ± 0.086 |   4.492 |   4.770 |      1.7 |

| Command |      Mean [s] | Min [s] | Max [s] |    Relative |
|:--------|--------------:|--------:|--------:|------------:|
| `cargo` | 1.251 ± 0.041 |   1.218 |   1.335 |        1.00 |
| `gcc`   | 1.253 ± 0.022 |   1.228 |   1.289 | 1.00 ± 0.04 |
| `musl`  | 2.791 ± 0.027 |   2.766 |   2.833 | 2.23 ± 0.08 |

| Command |      Mean [s] | Min [s] | Max [s] |    Relative |
|:--------|--------------:|--------:|--------:|------------:|
| `cargo` | 1.509 ± 0.019 |   1.487 |   1.542 |        1.00 |
| `gcc`   | 1.577 ± 0.031 |   1.542 |   1.614 | 1.04 ± 0.02 |
| `musl`  | 5.093 ± 0.044 |   4.997 |   5.166 | 3.38 ± 0.05 |

* clean2

| Command |      Mean [s] | Min [s] | Max [s] | Relative |
|:--------|--------------:|--------:|--------:|---------:|
| cargo   | 5.152 ± 0.026 |   5.132 |   5.221 |      1.1 |
| gcc     | 4.821 ± 0.003 |   4.817 |   4.826 |      1.0 |
| musl    | 5.983 ± 0.924 |   3.370 |   6.544 |      1.2 |

| Command |      Mean [ms] | Min [ms] | Max [ms] |    Relative |
|:--------|---------------:|---------:|---------:|------------:|
| `cargo` |    828.6 ± 6.9 |    817.6 |    838.1 |        1.00 |
| `gcc`   |   873.9 ± 15.7 |    857.5 |    915.8 | 1.05 ± 0.02 |
| `musl`  | 2198.2 ± 650.3 |   1468.9 |   3210.7 | 2.65 ± 0.79 |

| Command |     Mean [ms] | Min [ms] | Max [ms] |    Relative |
|:--------|--------------:|---------:|---------:|------------:|
| `cargo` |  905.9 ± 14.4 |    882.9 |    926.7 | 1.00 ± 0.03 |
| `gcc`   |  905.2 ± 28.2 |    862.8 |    943.4 |        1.00 |
| `musl`  | 2205.6 ± 20.0 |   2178.5 |   2250.0 | 2.44 ± 0.08 |

* connect

| Command |    Mean [ms] | Min [ms] | Max [ms] | Relative |
|:--------|-------------:|---------:|---------:|---------:|
| cargo   |  387.0 ± 0.7 |    386.2 |    387.8 |      1.0 |
| gcc     | 374.5 ± 74.3 |    247.8 |    532.1 |      1.0 |
| musl    |  383.0 ± 0.6 |    382.0 |    384.2 |      1.0 |

| Command |   Mean [ms] | Min [ms] | Max [ms] |    Relative |
|:--------|------------:|---------:|---------:|------------:|
| `cargo` |  83.8 ± 4.2 |     79.0 |    102.0 |        1.00 |
| `gcc`   | 118.5 ± 7.9 |    107.4 |    135.2 | 1.41 ± 0.12 |
| `musl`  | 131.6 ± 3.9 |    125.5 |    138.9 | 1.57 ± 0.09 |

| Command |    Mean [ms] | Min [ms] | Max [ms] |    Relative |
|:--------|-------------:|---------:|---------:|------------:|
| `cargo` |   86.0 ± 3.4 |     79.6 |     92.5 |        1.00 |
| `gcc`   | 123.9 ± 10.7 |    108.6 |    146.0 | 1.44 ± 0.14 |
| `musl`  | 162.7 ± 13.0 |    150.4 |    189.2 | 1.89 ± 0.17 |

* filter

| Command |   Mean [ms] | Min [ms] | Max [ms] | Relative |
|:--------|------------:|---------:|---------:|---------:|
| cargo   |  59.2 ± 1.1 |     58.4 |     66.2 |      1.1 |
| gcc     |  54.7 ± 0.3 |     54.2 |     55.7 |      1.0 |
| musl    | 55.6 ± 22.3 |     30.9 |    126.5 |      1.0 |

| Command |  Mean [ms] | Min [ms] | Max [ms] |    Relative |
|:--------|-----------:|---------:|---------:|------------:|
| `cargo` | 14.8 ± 0.9 |     13.4 |     20.4 |        1.00 |
| `gcc`   | 48.7 ± 2.6 |     41.5 |     56.6 | 3.29 ± 0.27 |
| `musl`  | 49.9 ± 2.6 |     44.9 |     61.0 | 3.37 ± 0.28 |

| Command |  Mean [ms] | Min [ms] | Max [ms] |    Relative |
|:--------|-----------:|---------:|---------:|------------:|
| `cargo` | 17.8 ± 1.4 |     15.2 |     22.4 |        1.00 |
| `gcc`   | 61.0 ± 9.5 |     45.9 |     80.3 | 3.43 ± 0.59 |
| `musl`  | 68.1 ± 7.7 |     53.8 |     80.9 | 3.83 ± 0.52 |
