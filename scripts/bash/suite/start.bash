# reset configs
renew_file $SUITE_GENERATED_ENV_FILE

# setup admin
admin_wallet=$(eth_new_wallet)
export ADMIN_ADDRESS=$(echo $admin_wallet | awk -F' ' '{ print $1 }')
export ADMIN_PRIVATE_KEY=$(echo $admin_wallet | awk -F' ' '{ print $2 }')
echo "ADMIN_ADDRESS=$ADMIN_ADDRESS" >> $SUITE_GENERATED_ENV_FILE
echo "ADMIN_PRIVATE_KEY=$ADMIN_PRIVATE_KEY" >> $SUITE_GENERATED_ENV_FILE

# start dfx
dfx_start $BASE_PORT
export DFX_URL="http://localhost:$BASE_PORT"
echo "DFX_URL=$DFX_PORT" >> $SUITE_GENERATED_ENV_FILE

# start chains
chains=$(cat $SUITE_CHAINS_LIST_FILE)
port_offset=1
while read -r chain; do
    rpc_port=$(( BASE_PORT + port_offset ))
    ssl_port=$(( BASE_PORT + port_offset + 1 ))

    eth_start_chain $chain $rpc_port
    ssl_start_proxy $rpc_port $ssl_port
    eth_fund_account $chain $rpc_port $ADMIN_ADDRESS

    chain=$(echo $chain | tr '[:lower:]' '[:upper:]')
    export "${chain}_RPC_URL=http://localhost:$rpc_port"
    export "${chain}_RPC_SSL_URL=https://localhost:$ssl_port"
    echo "${chain}_RPC_URL=http://localhost:$rpc_port" >> $SUITE_GENERATED_ENV_FILE
    echo "${chain}_RPC_SSL_URL=https://localhost:$ssl_port" >> $SUITE_GENERATED_ENV_FILE

    port_offset=$(( port_offset + 2 ))
done <<< "$chains"

echo "Suite ready." | pretty_log bash

wait