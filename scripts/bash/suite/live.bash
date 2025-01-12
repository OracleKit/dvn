renew_file $SUITE_GENERATED_ENV_FILE

admin_address=$1
admin_private_key=$2
if [ -z $admin_private_key ] || [ -z $admin_address ]; then
    echo "Usage: 'ADMIN_ADDRESS' 'ADMIN_PRIVATE_KEY'"
    terminate 1
fi

chains=$(cat $SUITE_CHAINS_LIST_FILE)

while read -r chain; do
    chain=$(echo $chain | tr '[:lower:]' '[:upper:]')
    rpc_url=$(eth_get_chain_env $chain "FORK_URL")

    echo "${chain}_RPC_URL=$rpc_url" >> $SUITE_GENERATED_ENV_FILE
done <<< "$chains"

echo "ADMIN_PRIVATE_KEY=$admin_private_key" >> $SUITE_GENERATED_ENV_FILE
echo "ADMIN_ADDRESS=$admin_address" >> $SUITE_GENERATED_ENV_FILE