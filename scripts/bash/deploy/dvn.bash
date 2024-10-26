set -e

DIRNAME=$(dirname $(dirname $0))
source $DIRNAME/common.bash
source $DIRNAME/dfx.bash
source $DIRNAME/eth.bash

source_env $USER_CONFIG_ENV_FILE
source_env $SUITE_GENERATED_ENV_FILE
source_env $SUITE_CHAINS_ENV_FILE

deploy_dvn
dvn_address=$(get_dvn_address)
chains=$(cat $SUITE_CHAINS_LIST_FILE)

while IFS= read -r chain; do
    chain_rpc_env_name="$(echo $chain | tr '[:lower:]' '[:upper:]')_RPC_URL"
    chain_rpc_url=$(echo ${!chain_rpc_env_name})
    chain_port=$(echo $chain_rpc_url | awk -F':' '{ print $3 }' | cut -c1-4)
    fund_account $chain $chain_port $dvn_address
done <<< "$chains"