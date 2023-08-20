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

hyperfine --warmup 1 --export-markdown rgr.sort.md.tmp \
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
    '

cat rgr.sort.md.tmp

```
