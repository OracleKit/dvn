set -e

DIRNAME=$(dirname $(dirname $0))
source $DIRNAME/common.bash

setup_trap_handlers

export POCKET_IC_BIN=$(pwd)/.sink/bin/pocket-ic
cargo test