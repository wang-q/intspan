# `far` and `faops`

```bash
brew install intspan
brew install faops

brew install hyperfine

```

## `far size`

```bash
hyperfine --warmup 1 --export-markdown size.md.tmp \
    'cat tests/far/ufasta.fa | far   size stdin > /dev/null' \
    'cat tests/far/ufasta.fa | faops size stdin > /dev/null' \
    'far   size tests/far/ufasta.fa.gz > /dev/null' \
    'faops size tests/far/ufasta.fa.gz > /dev/null'

```

| Command                   | Mean [ms] | Min [ms] | Max [ms] |    Relative |
|:--------------------------|----------:|---------:|---------:|------------:|
| `far   size ufasta.fa`    | 2.8 ± 0.3 |      2.2 |      5.2 | 1.11 ± 0.18 |
| `faops size ufasta.fa`    | 2.7 ± 0.3 |      1.9 |      3.8 | 1.09 ± 0.18 |
| `far   size ufasta.fa.gz` | 2.8 ± 0.4 |      2.1 |      4.7 | 1.11 ± 0.20 |
| `faops size ufasta.fa.gz` | 2.5 ± 0.3 |      2.0 |      3.5 |        1.00 |

| Command                   | Mean [ms] | Min [ms] | Max [ms] |    Relative |
|:--------------------------|----------:|---------:|---------:|------------:|
| `far   size ufasta.fa`    | 1.7 ± 0.2 |      1.2 |      2.7 | 1.07 ± 0.16 |
| `faops size ufasta.fa`    | 1.7 ± 0.2 |      1.3 |      3.5 | 1.09 ± 0.15 |
| `far   size ufasta.fa.gz` | 1.7 ± 0.2 |      1.3 |      2.6 | 1.06 ± 0.15 |
| `faops size ufasta.fa.gz` | 1.6 ± 0.2 |      1.2 |      2.5 |        1.00 |

## `far some`

```bash
hyperfine --warmup 1 --export-markdown some.md.tmp \
    'far   some tests/far/ufasta.fa.gz tests/far/lst.txt > /dev/null' \
    'faops some tests/far/ufasta.fa.gz tests/far/lst.txt stdout > /dev/null' \
    'far   some -i tests/far/ufasta.fa.gz tests/far/lst.txt > /dev/null' \
    'faops some -i tests/far/ufasta.fa.gz tests/far/lst.txt stdout > /dev/null'

```

| Command       | Mean [ms] | Min [ms] | Max [ms] |    Relative |
|:--------------|----------:|---------:|---------:|------------:|
| far   some    | 4.2 ± 0.4 |      3.2 |      5.7 | 1.00 ± 0.14 |
| faops some    | 4.2 ± 0.4 |      3.4 |      5.6 | 1.00 ± 0.13 |
| far   some -i | 4.2 ± 0.4 |      3.4 |      6.1 |        1.00 |
| faops some -i | 4.2 ± 0.4 |      3.4 |      5.7 | 1.00 ± 0.13 |

| Command       | Mean [ms] | Min [ms] | Max [ms] |    Relative |
|:--------------|----------:|---------:|---------:|------------:|
| far   some    | 2.4 ± 0.2 |      2.0 |      3.5 | 1.03 ± 0.11 |
| faops some    | 2.3 ± 0.2 |      1.8 |      3.4 |        1.00 |
| far   some -i | 2.3 ± 0.2 |      1.8 |      3.6 | 1.01 ± 0.13 |
| faops some -i | 2.3 ± 0.2 |      1.8 |      3.6 | 1.01 ± 0.12 |
