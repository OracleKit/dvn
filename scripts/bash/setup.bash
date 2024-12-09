set -e

DIRNAME=$(dirname $0)
source $DIRNAME/common.bash
source $DIRNAME/dfx.bash

setup_directories

if [ ! -f $SINK_BIN_DIR/pocket-ic ]; then
    dfx_setup_pocketic_bin
fi
