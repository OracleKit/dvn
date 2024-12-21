npx tsx ./scripts/ts/deploy.ts $(cat $SUITE_CHAINS_LIST_FILE | xargs)
source_env $SUITE_GENERATED_ENV_FILE

chains=$(cat $SUITE_CHAINS_LIST_FILE)

while IFS= read -r chain; do
    chain_upper_caps=$(echo $chain | tr '[:lower:]' '[:upper:]')
    chain_rpc_ssl_url=$(eth_get_chain_env $chain_upper_caps "RPC_SSL_URL")
    chain_id=$(eth_get_chain_env $chain_upper_caps "CHAIN_ID")
    chain_endpoint_id=$(eth_get_chain_env $chain_upper_caps "ENDPOINT_ID")
    chain_dvn_address=$(eth_get_chain_env $chain_upper_caps "DVN_ADDRESS")
    
    dfx_add_dvn_chain $chain_rpc_ssl_url $chain_id $chain_endpoint_id $chain_dvn_address
done <<< "$chains"
