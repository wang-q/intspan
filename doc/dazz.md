# Comparison of `wfmash` with `App::Dazz`

wfmash
dazz - daligner

```shell
curl -LO https://github.com/waveygang/wfmash/releases/download/v0.15.0/wfmash
mv wfmash ~/bin/

brew install --HEAD wang-q/tap/dazz_db
brew install --HEAD wang-q/tap/daligner
cpanm -nq https://github.com/wang-q/App-Dazz.git

```

## overlap

```shell

dazz overlap tests/ovlpr/1_4.pac.fasta -o stdout | sort

wfmash --map-pct-id 70 --segment-length 500 --skip-self \
    tests/ovlpr/1_4.pac.fasta |
    perl -nlp -e 's/\tcg.+$//g' |
    sort

```

wfmash is very inaccurate about the start and end of the alignment and cannot be used to determine
unitig relationships.
