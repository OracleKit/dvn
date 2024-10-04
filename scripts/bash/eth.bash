function setup_admin_wallet {
    out=$(cast wallet new)
    export ETH_ADMIN_ADDRESS=$( echo "$out" | grep "Address" | awk '{ print $2 }' )
    export ETH_ADMIN_PRIVATE_KEY=$( echo "$out" | grep "Private" | awk '{ print $3 }' )
}

function get_admin_wallet_address {
    echo $ETH_ADMIN_ADDRESS
}

function get_admin_wallet_private_key {
    echo $ETH_ADMIN_PRIVATE_KEY
}

# Usage: [CHAIN_ID] [FORK_URL] [FORK_BLOCK_NUM] [RPC_PORT]
function start_network {
    anvil --port $4 --auto-impersonate \
        --chain-id $1 --fork-url $2 --fork-block-number $3 \
        --accounts 1 2>&1 & 
    
    sleep 5
    rich_wallet=$(cast rpc --rpc-url "localhost:$4" eth_accounts | awk -F'"' '{ print $2 }')
    cast send --rpc-url "localhost:$4" --unlocked --from $rich_wallet --value 100ether $(get_admin_wallet_address)

    echo "Started network"

    wait
}
