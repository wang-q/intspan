# `rgr`

## Test materials

```shell
cd ~/gars

redis-server &

gars env

gars status drop
gars gen genome/genome.fa.gz --piece 500000

gars range features/T-DNA.CSHL.rg
gars range features/T-DNA.FLAG.rg
gars range features/T-DNA.MX.rg
gars range features/T-DNA.RATM.rg

gars tsv -s "range:*" | gzip -9 > ranges.tsv.gz

gzip -dcf ranges.tsv.gz | wc -l
#102973

mv ranges.tsv.gz ~/Scripts/intspan/tests/rgr/

```

## `rgr sort`

```shell
cd ~/Scripts/intspan/

hyperfine --warmup 1  \
    -n 'sort' \
    '
    rgr sort -H tests/rgr/ranges.tsv.gz tests/rgr/ranges.tsv.gz tests/rgr/ranges.tsv.gz
    ' \
    -n 'sort -f' \
    '
    rgr sort -H -f 5 tests/rgr/ranges.tsv.gz tests/rgr/ranges.tsv.gz tests/rgr/ranges.tsv.gz
    ' \
    -n 'sort -g' \
    '
    rgr sort -H -f 5 -g 6 tests/rgr/ranges.tsv.gz tests/rgr/ranges.tsv.gz tests/rgr/ranges.tsv.gz
    ' \
    --export-markdown rgr.sort.md.tmp

cat rgr.sort.md.tmp

```

| Command   |    Mean [ms] | Min [ms] | Max [ms] |    Relative |
|:----------|-------------:|---------:|---------:|------------:|
| `sort`    |  621.0 ± 3.3 |    617.1 |    626.3 | 1.01 ± 0.08 |
| `sort -f` |  629.4 ± 3.0 |    625.3 |    635.0 | 1.02 ± 0.08 |
| `sort -g` | 615.0 ± 50.4 |    471.9 |    636.2 |        1.00 |

## `rgr filter`

```shell
cd ~/Scripts/intspan/

hyperfine --warmup 1 \
    -n 'rgr filter' \
    '
    rgr filter tests/rgr/ctg_2_1_.gc.tsv --str-eq 3:1 > /dev/null
    ' \
    -n 'tsv-filter' \
    '
    tsv-filter tests/rgr/ctg_2_1_.gc.tsv --str-eq 3:1 > /dev/null
    ' \
    --export-markdown rgr.filter.md.tmp

cat rgr.filter.md.tmp

```

| Command      |  Mean [ms] | Min [ms] | Max [ms] |    Relative |
|:-------------|-----------:|---------:|---------:|------------:|
| `rgr filter` | 10.5 ± 1.1 |      9.3 |     14.6 | 2.25 ± 0.91 |
| `tsv-filter` |  4.7 ± 1.8 |      2.3 |      7.9 |        1.00 |

## `rgr select`

```shell
cd ~/Scripts/intspan/

hyperfine --warmup 1 \
    -n 'rgr filter' \
    '
    rgr select tests/rgr/ctg_2_1_.gc.tsv -f 1,3 > /dev/null
    ' \
    -n 'tsv-filter' \
    '
    tsv-select tests/rgr/ctg_2_1_.gc.tsv -f 1,3 > /dev/null
    ' \
    -n 'rgr filter -H' \
    '
    rgr select tests/rgr/ctg_2_1_.gc.tsv -H -f "#range,signal" > /dev/null
    ' \
    -n 'tsv-filter -H' \
    '
    tsv-select tests/rgr/ctg_2_1_.gc.tsv -H -f "#range,signal" > /dev/null
    ' \
    --export-markdown rgr.select.md.tmp

cat rgr.select.md.tmp

```

| Command         |  Mean [ms] | Min [ms] | Max [ms] |    Relative |
|:----------------|-----------:|---------:|---------:|------------:|
| `rgr filter`    | 13.9 ± 0.7 |     12.9 |     17.8 | 2.54 ± 0.86 |
| `tsv-filter`    |  5.6 ± 1.9 |      3.7 |     10.1 | 1.01 ± 0.49 |
| `rgr filter -H` | 13.9 ± 0.8 |     12.9 |     17.5 | 2.53 ± 0.86 |
| `tsv-filter -H` |  5.5 ± 1.8 |      3.6 |     10.1 |        1.00 |

## Sampling

```shell
cd ~/Scripts/intspan/

hyperfine --warmup 1 \
    -n 'tsv-sample' \
    '
    tsv-sample tests/rgr/ctg_2_1_.gc.tsv --prob 0.4 > /dev/null
    ' \
    -n 'qsv sample' \
    '
    qsv sample 0.4 tests/rgr/ctg_2_1_.gc.tsv > /dev/null
    ' \
    --export-markdown rgr.sample.md.tmp

cat rgr.sample.md.tmp


```

| Command      |    Mean [ms] | Min [ms] | Max [ms] |    Relative |
|:-------------|-------------:|---------:|---------:|------------:|
| `tsv-sample` |   14.0 ± 1.4 |     10.8 |     19.8 |        1.00 |
| `qsv sample` | 127.4 ± 14.5 |    111.5 |    165.6 | 9.13 ± 1.38 |
