set -e

DIRNAME=$(dirname $(dirname $0))
source $DIRNAME/eth.bash
source $DIRNAME/dfx.bash
source $DIRNAME/ssl.bash
source $DIRNAME/log.bash
source $DIRNAME/common.bash

src_chain_name="polygonAmoy"
dest_chain_name="ethereumHolesky"

src_chain_name_caps=$(echo $src_chain_name | tr [:lower:] [:upper:])
dest_chain_name_caps=$(echo $dest_chain_name | tr [:lower:] [:upper:])

# Load configs
source_env $USER_ENV_FILE

src_chain_id=$(eth_get_chain_env $src_chain_name_caps "CHAIN_ID")
src_chain_eid=$(eth_get_chain_env $src_chain_name_caps "ENDPOINT_ID")
dest_chain_id=$(eth_get_chain_env $dest_chain_name_caps "CHAIN_ID")
dest_chain_eid=$(eth_get_chain_env $dest_chain_name_caps "ENDPOINT_ID")

setup_trap_handlers

# Setup suite directories and env files
setup_directories
renew_file $SUITE_GENERATED_ENV_FILE
pretty_log_clear

# setup admin wallet
admin_wallet=$(eth_new_wallet)
admin_address=$(echo $admin_wallet | awk -F' ' '{ print $1 }')
admin_private_key=$(echo $admin_wallet | awk -F' ' '{ print $2 }')


# setup chains
eth_start_chain $src_chain_name $(( BASE_PORT + 1 ))
eth_start_chain $dest_chain_name $(( BASE_PORT + 2 ))
ssl_start_proxy $(( BASE_PORT + 1 )) $(( BASE_PORT + 3 ))
ssl_start_proxy $(( BASE_PORT + 2 )) $(( BASE_PORT + 4 ))

src_chain_rpc_url="http://127.0.0.1:$(( BASE_PORT + 1 ))/"
src_chain_rpc_ssl_url="https://127.0.0.1:$(( BASE_PORT + 3 ))/"
dest_chain_rpc_url="http://127.0.0.1:$(( BASE_PORT + 2 ))/"
dest_chain_rpc_ssl_url="https://127.0.0.1:$(( BASE_PORT + 4 ))/"

# fund admin
eth_fund_account $src_chain_name $(( BASE_PORT + 1 )) $admin_address
eth_fund_account $dest_chain_name $(( BASE_PORT + 2 )) $admin_address

# deploy dvn and oapp
export ADMIN_ADDRESS=$admin_address
export ADMIN_PRIVATE_KEY=$admin_private_key
export "${src_chain_name_caps}_RPC_URL=$src_chain_rpc_url"
export "${dest_chain_name_caps}_RPC_URL=$dest_chain_rpc_url"
npx ts-node ./scripts/ts/deploy.ts $src_chain_name $dest_chain_name

source $SUITE_GENERATED_ENV_FILE
src_chain_dvn_address=$(eth_get_chain_env $src_chain_name_caps "DVN_ADDRESS")
src_chain_oapp_address=$(eth_get_chain_env $src_chain_name_caps "OAPP_ADDRESS")
dest_chain_dvn_address=$(eth_get_chain_env $dest_chain_name_caps "DVN_ADDRESS")
dest_chain_oapp_address=$(eth_get_chain_env $dest_chain_name_caps "OAPP_ADDRESS")

# start chains
dfx_start $BASE_PORT
dfx_deploy_dvn
dfx_add_dvn_chain $src_chain_rpc_ssl_url $src_chain_id $src_chain_eid $src_chain_dvn_address
dfx_add_dvn_chain $dest_chain_rpc_ssl_url $dest_chain_id $dest_chain_eid $dest_chain_dvn_address

# fund dvn
dvn_address=$(dfx_get_dvn_address)
eth_fund_account $src_chain_name $(( BASE_PORT + 1 )) $dvn_address
eth_fund_account $dest_chain_name $(( BASE_PORT + 2 )) $dvn_address

echo "Suite ready." | pretty_log_term bash

export "${src_chain_name_caps}_DVN_ADDRESS=$src_chain_dvn_address"
export "${src_chain_name_caps}_OAPP_ADDRESS=$src_chain_oapp_address"
export "${dest_chain_name_caps}_DVN_ADDRESS=$dest_chain_dvn_address"
export "${dest_chain_name_caps}_OAPP_ADDRESS=$dest_chain_oapp_address"
export SOURCE_CHAIN_NAME=$src_chain_name
export DESTINATION_CHAIN_NAME=$dest_chain_name
npx mocha