DIRNAME=$(dirname $0)
source $DIRNAME/eth.bash
source $DIRNAME/dfx.bash
source $DIRNAME/ssl.bash
source $DIRNAME/log.bash

BASE_PORT=9000

function get_chain_env {
    chain_var_name="${1}_${2}"
    echo ${!chain_var_name}
}

function setup_chains {
    while IFS=' ' read -r chain; do
        rpc_port=$BASE_PORT
        ssl_port=$(( rpc_port + 1 ))
        fork_url=$( get_chain_env $chain FORK_URL )
        fork_block_num=$( get_chain_env $chain FORK_BLOCK_NUM )
        chain_id=$( get_chain_env $chain CHAIN_ID )

        start_network $chain_id $fork_url $fork_block_num $rpc_port 2>&1 | pretty_log $chain &
        start_ssl_proxy $rpc_port $ssl_port 2>&1 | pretty_log $chain &

        echo "${chain}_RPC_SSL_URL=https://localhost:${ssl_port}/" >> .env.generated
        echo "${chain}_RPC_URL=http://localhost:${rpc_port}/" >> .env.generated

        BASE_PORT=$(( BASE_PORT + 2 ))
    done

    wait
}

function fund_dvn {
    while IFS=' ' read -r chain; do
        rpc_url=$( get_chain_env $chain RPC_URL )
        echo "Funding $chain"
        cast send --rpc-url $rpc_url --private-key $ADMIN_PRIVATE_KEY --value 10ether $DVN_ADDRESS
    done
}

rm .env.generated
setup_admin_wallet
echo "ADMIN_ADDRESS=$(get_admin_wallet_address)" >> .env.generated
echo "ADMIN_PRIVATE_KEY=$(get_admin_wallet_private_key)" >> .env.generated

export $(cat .env.local | xargs)

start_dfx 2>&1 | pretty_log dfx &
CHAINS=$(cat .env.local | grep CHAIN_ID | awk -F'_' '{ print $1 }')
echo "$CHAINS" | setup_chains &
sleep 10

export $(cat .env.generated | xargs)
npx ts-node ./scripts/ts/deploy.ts >> .env.generated
export $(cat .env.generated | xargs)
deploy_dvn | pretty_log dfx

export DVN_ADDRESS=$(dfx canister call dvn public_key | awk -F'"' '{ print $2 }')
echo "DVN_ADDRESS=${DVN_ADDRESS}" >> .env.generated
echo "DVN Address: ${DVN_ADDRESS}"

echo "$CHAINS" | fund_dvn

wait