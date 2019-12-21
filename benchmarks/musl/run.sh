#!/usr/bin/env bash

#----------------------------#
# Colors in term
#----------------------------#
# http://stackoverflow.com/questions/5947742/how-to-change-the-output-color-of-echo-in-linux
GREEN=
RED=
NC=
if tty -s < /dev/fd/1 2> /dev/null; then
    GREEN='\033[0;32m'
    RED='\033[0;31m'
    NC='\033[0m' # No Color
fi

log_warn () {
    echo >&2 -e "${RED}==> $@ <==${NC}"
}

log_info () {
    echo >&2 -e "${GREEN}==> $@${NC}"
}

log_debug () {
    echo >&2 -e "==> $@"
}

#----------------------------#
# Prepare
#----------------------------#
COMMAND_TIME="command time -v"
if [[ `uname` == 'Darwin' ]]; then
    COMMAND_TIME="command time -l"
fi

# enter BASE_DIR
BASE_DIR=$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )
cd ${BASE_DIR}

if [[ ! -e intspan-x86_64-unknown-linux-gnu.tar.gz ]]; then
    wget https://github.com/wang-q/intspan/releases/download/v0.4.9/intspan-x86_64-unknown-linux-gnu.tar.gz
fi

if [[ ! -e intspan-x86_64-unknown-linux-gnu.tar.gz ]]; then
    wget -N https://github.com/wang-q/intspan/releases/download/v0.4.9/intspan-x86_64-unknown-linux-musl.tar.gz
fi

tar xvfz intspan-x86_64-unknown-linux-gnu.tar.gz
tar xvfz intspan-x86_64-unknown-linux-musl.tar.gz

#----------------------------#
# Run
#----------------------------#
log_info "sort"
hyperfine --warmup 1 --export-markdown sort.md \
    'gzip -dcf ../../tests/Atha/links.lastz.tsv.gz tests/Atha/links.blast.tsv.gz | target/release/linkr                           sort stdin -o /dev/null' \
    'gzip -dcf ../../tests/Atha/links.lastz.tsv.gz tests/Atha/links.blast.tsv.gz | target/x86_64-unknown-linux-musl/release/linkr sort stdin -o /dev/null'

echo >&2

log_info "clean"
hyperfine --warmup 1 --export-markdown clean.md \
    'target/release/linkr                           clean ../../tests/Atha/sort.tsv -o /dev/null' \
    'target/x86_64-unknown-linux-musl/release/linkr clean ../../tests/Atha/sort.tsv -o /dev/null'

echo >&2

log_info "merge"
hyperfine --warmup 1 --export-markdown merge.md \
    'target/release/linkr                           merge ../../tests/Atha/sort.clean.tsv -c 0.95 -o /dev/null' \
    'target/x86_64-unknown-linux-musl/release/linkr merge ../../tests/Atha/sort.clean.tsv -c 0.95 -o /dev/null'

echo >&2

log_info "clean2"
hyperfine --warmup 1 --export-markdown clean2.md \
    'target/release/linkr                           clean ../../tests/Atha/sort.clean.tsv -r ../../tests/Atha/merge.tsv --bundle 500 -o /dev/null' \
    'target/x86_64-unknown-linux-musl/release/linkr clean ../../tests/Atha/sort.clean.tsv -r ../../tests/Atha/merge.tsv --bundle 500 -o /dev/null'

echo >&2

log_info "connect"
hyperfine --warmup 1 --export-markdown connect.md \
    'target/release/linkr                           connect ../../tests/Atha/clean.tsv -o /dev/null' \
    'target/x86_64-unknown-linux-musl/release/linkr connect ../../tests/Atha/clean.tsv -o /dev/null'

echo >&2

log_info "filter"
hyperfine --warmup 1 --export-markdown filter.md \
    'target/release/linkr                           filter ../../tests/Atha/connect.tsv -r 0.8 -o /dev/null' \
    'target/x86_64-unknown-linux-musl/release/linkr filter ../../tests/Atha/connect.tsv -r 0.8 -o /dev/null'

echo >&2
