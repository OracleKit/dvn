set -e

DIRNAME=$(dirname $0)
source $DIRNAME/common.bash
source $DIRNAME/dfx.bash

alchemy_api_key=$1
if [ -z "$alchemy_api_key" ]; then
    echo "No Alchemy API key provided"
    exit
fi

setup_directories

if [ ! -f $SINK_BIN_DIR/pocket-ic ]; then
    dfx_setup_pocketic_bin
fi

cat .env.template | sed "s/##ALCHEMY_API_KEY##/$alchemy_api_key/g" > .env.local