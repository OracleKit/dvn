# Setups local test env

_LOGS_DIR=./logs

# Usage: [COMPONENT_NAME] => [PATH relative to project dir]
function _get_logs_file_path {
    echo "$_LOGS_DIR/$1.txt"
}

# Usage: [COMPONENT_NAME]
function _log_output {
    PREFIX=$1
    LOG_FILE=$(_get_logs_file_path $1)
    while IFS= read -r line; do
        echo $line >> $LOG_FILE
        echo "[" $PREFIX "]" $line
    done
}

function setup_logs_dir {
    rm -rf $_LOGS_DIR 2>&1
    mkdir $_LOGS_DIR
}

# Usage: [RPC_PORT] [NETWORK_NAME] [FORK_RPC_URL] [FORK_BLOCK_NUM]
function setup_hardhat_network {
    npx hardhat node --port $1 --fork $3 --fork-block-number $4 2>&1 | _log_output $2 &
}

function setup_dvn {
    dfx stop 2>&1 | _log_output dfx
    dfx start --clean 2>&1 | _log_output dfx &
    sleep 5
    dfx deploy dvn 2>&1 | _log_output dfx
}

# Usage: [NETWORK_NAME]
function get_rich_private_key {
    echo "Extracting rich private key..." | _log_output bash
    RICH_PRIVATE_KEY=$(cat $(_get_logs_file_path $1) | grep -m 1 "Private" | awk '{ print $3 }')
}

function get_dvn_address {
    echo "Getting DVN Address..." | _log_output bash
    OUT=$(dfx canister call dvn public_key)
    DVN_ADDRESS=$(echo $OUT | cut -d\" -f 2)
}

# Usage: [SOURCE_NETWORK_PORT] [TARGET_NETWORK_PORT]
function run_setup_script {
    npx ts-node ./scripts/ts/setup.ts 9000 $RICH_PRIVATE_KEY $DVN_ADDRESS 2>&1 | _log_output bash
}

# Usage: [APP_PORT] [SSL_PORT] [PROXY_NAME]
function setup_ssl_proxy {
    npx local-ssl-proxy --source $2 --target $1 --cert localhost.pem --key localhost-key.pem 2>&1 | _log_output $3 &
}

function test {
    while IFS= read -r line; do
        KEY=$(echo $line | awk -F'=' '{ print $1 }')
        VALUE=$(echo $line | awk -F'=' '{ print $2 }')
        export $KEY="$VALUE"
        env
    done
}

# source ./.env.local
# echo $SOURCE_NETWORK_PORT
# export AAAA=BBB
echo $(cat ./.env.local | xargs)
# env
# npx ts-node ./scripts/ts/setup.ts

# setup_logs_dir
# setup_hardhat_network $SOURCE_NETWORK_PORT hardhat_source $SOURCE_NETWORK_FORK_URL $SOURCE_NETWORK_FORK_BLOCK
# setup_hardhat_network $TARGET_NETWORK_PORT hardhat_target $TARGET_NETWORK_FORK_URL $TARGET_NETWORK_FORK_BLOCK
# setup_dvn
# setup_ssl_proxy $SOURCE_NETWORK_PORT $SOURCE_NETWORK_SSL_PORT
# setup_ssl_proxy $TARGET_NETWORK_PORT $TARGET_NETWORK_SSL_PORT

# # letting everything setup
# sleep 1

# get_rich_private_key
# get_dvn_address
# run_setup_script
# echo "Setup complete!"

# wait