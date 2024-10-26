DIRNAME=$(dirname $(dirname $0))
source $DIRNAME/common.bash

setup_trap_handlers

$SINK_BIN_DIR/pocket-ic -p 49462 >/dev/null 2>&1 &
export POCKET_IC_URL=http://localhost:49462

npx mocha --import=tsx

terminate