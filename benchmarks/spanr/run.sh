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
log_info "jrunlist"
${COMMAND_TIME} jrunlist \
    statop \
    chr.sizes sep-gene.yml paralog.yml  \
    --op intersect --all \
    -o stdout \
    > jstatop.csv.tmp

log_info "spanr"
${COMMAND_TIME} spanr \
    statop \
    chr.sizes sep-gene.yml paralog.yml  \
    --op intersect --all \
    -o stdout \
    > rstatop.csv.tmp

log_info "App::RL"
${COMMAND_TIME} runlist \
    stat2 \
    -s chr.sizes sep-gene.yml paralog.yml  \
    --op intersect --all --mk \
    -o stdout \
    > pstatop.csv.tmp
echo >&2
