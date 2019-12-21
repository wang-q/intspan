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

#----------------------------#
# Run
#----------------------------#
log_warn "merge"
log_info "jrange merge lastz blast"
${COMMAND_TIME} jrange \
    merge \
    -o stdout -c 0.95 \
    links.lastz.tsv \
    links.blast.tsv |
    sort \
    > jmerge.tsv.tmp

log_info "linkr merge lastz blast"
${COMMAND_TIME} linkr \
    merge \
    -o stdout -c 0.95 \
    links.lastz.tsv \
    links.blast.tsv |
    sort \
    > rmerge.tsv.tmp

log_info "rangeops merge lastz blast"
${COMMAND_TIME} rangeops \
    merge \
    -o stdout -c 0.95 -p 8 \
    links.lastz.tsv \
    links.blast.tsv |
    sort \
    > pmerge.tsv.tmp
echo >&2

log_warn "clean"
log_info "jrange clean sort.clean"
${COMMAND_TIME} jrange \
    clean \
    -o stdout \
    sort.clean.tsv \
    > jclean.tsv.tmp

log_info "linkr clean sort.clean"
${COMMAND_TIME} linkr \
    clean \
    -o stdout \
    sort.clean.tsv \
    > rclean.tsv.tmp

log_info "rangeops clean sort.clean"
${COMMAND_TIME} rangeops \
    clean \
    -o stdout \
    sort.clean.tsv \
    > pclean.tsv.tmp
echo >&2

log_warn "clean bundle"
log_info "jrange clean bundle sort.clean"
${COMMAND_TIME} jrange \
    clean \
    -o stdout \
    --bundle 500 \
    sort.clean.tsv \
    > jbundle.tsv.tmp

log_info "linkr clean bundle sort.clean"
${COMMAND_TIME} linkr \
    clean \
    -o stdout \
    --bundle 500 \
    sort.clean.tsv \
    > rbundle.tsv.tmp

log_info "rangeops clean bundle sort.clean"
${COMMAND_TIME} rangeops \
    clean \
    -o stdout \
    --bundle 500 \
    sort.clean.tsv \
    > pbundle.tsv.tmp
echo >&2
