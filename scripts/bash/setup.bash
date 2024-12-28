set -e
set -o pipefail

DIRNAME=$(dirname $0)
source $DIRNAME/common.bash
source $DIRNAME/dfx.bash

setup_directories

if [ ! -f $SINK_BIN_DIR/pocket-ic ]; then
    dfx_setup_pocketic_bin
fi

# Generate .env.local
if [ ! -f .env.local ]; then
    alchemy_api_key=$1

    if [ -z "$alchemy_api_key" ]; then
        echo "No Alchemy API key provided"
        exit
    fi

    cat .env.template | sed "s/##ALCHEMY_API_KEY##/$alchemy_api_key/g" > .env.local
fi

# Generate certs for local ssl proxy
if [ ! -f localhost.pem ] || [ ! -f localhost-key.pem ]; then
    mkcert localhost
fi

# Check for candid-extractor
if ! candid-extractor --version; then
    cargo install candid-extractor
fi