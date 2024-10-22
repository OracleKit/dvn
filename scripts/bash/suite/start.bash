DIRNAME=$(dirname $(dirname $0))
source $DIRNAME/eth.bash
source $DIRNAME/dfx.bash
source $DIRNAME/ssl.bash
source $DIRNAME/log.bash
source $DIRNAME/common.bash

# Load configs
source_env $USER_CONFIG_ENV_FILE

function setup_admin_wallet {
    wallet=$(new_wallet)
    address=$(echo $wallet | awk '{ print $1 }')
    private_key=$(echo $wallet | awk '{ print $2 }')

    echo "ADMIN_ADDRESS=$address" >> $SUITE_GENERATED_ENV_FILE
    echo "ADMIN_PRIVATE_KEY=$private_key" >> $SUITE_GENERATED_ENV_FILE

    source_env $SUITE_GENERATED_ENV_FILE
}

function start_chains {
    chains=$(cat $SUITE_CHAINS_LIST_FILE)
    port=$BASE_PORT

    while read -r chain; do
        start_chain $chain $port
        start_ssl_proxy $port $(( port + 1 ))
        fund_account $chain $port $ADMIN_ADDRESS

        chain=$(echo $chain | tr '[:lower:]' '[:upper:]')
        echo "$(echo $chain)_RPC_URL=http://localhost:$port" >> $SUITE_GENERATED_ENV_FILE
        echo "$(echo $chain)_RPC_SSL_URL=https://localhost:$(( port + 1 ))" >> $SUITE_GENERATED_ENV_FILE

        port=$(( port + 2 ))
    done <<< "$chains"

    source_env $SUITE_GENERATED_ENV_FILE
}

setup_trap_handlers

# Setup suite directories and env files
setup_directories
clear_logs
renew_file $SUITE_GENERATED_ENV_FILE
renew_file $DEPLOYED_ENV_LOCAL_FILE

source_env $SUITE_CHAINS_ENV_FILE
setup_admin_wallet
start_dfx
start_chains

wait