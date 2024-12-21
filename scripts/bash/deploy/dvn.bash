dfx_deploy_dvn
dvn_address=$(dfx_get_dvn_address)
dvn_canister_id=$(dfx_get_canister_id)
chains=$(cat $SUITE_CHAINS_LIST_FILE)

while IFS= read -r chain; do
    chain_rpc_env_name="$(echo $chain | tr '[:lower:]' '[:upper:]')_RPC_URL"
    chain_rpc_url=$(echo ${!chain_rpc_env_name})
    chain_port=$(echo $chain_rpc_url | awk -F':' '{ print $3 }' | cut -c1-4)
    eth_fund_account $chain $chain_port $dvn_address
done <<< "$chains"

echo "DVN_CANISTER_ADDRESS=$dvn_address" >> $SUITE_GENERATED_ENV_FILE
echo "DVN_CANISTER_ID=$dvn_canister_id" >> $SUITE_GENERATED_ENV_FILE