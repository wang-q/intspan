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

| Command            | Mean [ms] | Min [ms] | Max [ms] |    Relative |
|:-------------------|----------:|---------:|---------:|------------:|
| far   ufasta.fa    | 5.1 ± 0.9 |      4.2 |     15.9 | 2.12 ± 0.48 |
| faops ufasta.fa    | 2.4 ± 0.3 |      1.8 |      5.0 |        1.00 |
| far   ufasta.fa.gz | 4.9 ± 0.5 |      4.1 |      6.7 | 2.06 ± 0.35 |
| faops ufasta.fa.gz | 2.4 ± 1.1 |      1.6 |     18.5 | 1.00 ± 0.47 |
