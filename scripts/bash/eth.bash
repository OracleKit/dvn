source $DIRNAME/log.bash
source $DIRNAME/common.bash

function _get_chain_env {
    chain_var_name="${1}_${2}"
    echo ${!chain_var_name}
}

# Usage: [CHAIN_NAME]
function _wait_for_network_start {
    log_file=$(pretty_log_file $1)
    until grep "Listening" $log_file >/dev/null 2>&1; do
        sleep .1
    done
}

# Returns: [ADDRESS] [PRIV_KEY]
function new_wallet {
    out=$(cast wallet new)
    address=$(echo "$out" | grep "Address" | awk '{ print $2 }')
    private_key=$(echo "$out" | grep "Private" | awk '{ print $3 }')
    echo "$address $private_key"
}

# Usage: [CHAIN_NAME] [RPC_PORT]
function start_chain {
    echo "Starting local chain..." | pretty_log_term $1
    
    rpc_port=$2
    chain_name=$(echo $1 | tr '[:lower:]' '[:upper:]')
    chain_id=$(_get_chain_env $chain_name CHAIN_ID)
    fork_url=$(_get_chain_env $chain_name FORK_URL)
    fork_block_num=$(_get_chain_env $chain_name FORK_BLOCK_NUM)

    anvil --port $rpc_port --auto-impersonate \
        --chain-id $chain_id --fork-url $fork_url --fork-block-number $fork_block_num \
        --accounts 1 >$(pretty_log_file $1) 2>&1 & 
    
    _wait_for_network_start $1
    echo "Local chain started on port $rpc_port" | pretty_log_term $1
    tail -n0 -f $(pretty_log_file $1) | pretty_log_term $1 &
}

# Usage: [CHAIN_NAME] [RPC_PORT] [RECEIVER_ADDRESS]
function fund_account {
    echo "Funding account..." | pretty_log_term $1
    rich_wallet=$(cast rpc --rpc-url "localhost:$2" eth_accounts | awk -F'"' '{ print $2 }')
    cast send --rpc-url "localhost:$2" --unlocked --from $rich_wallet --value 100ether $3 >/dev/null

    if [ $? -eq 0 ]; then
        echo "Account funded" | pretty_log_term $1
    else
        echo "Account funding failed" | pretty_log_term $1
        terminate
    fi
}