function start_dfx {
    dfx stop 2>&1 > /dev/null
    dfx start --clean 2>&1 &
    sleep 5
    wait
}

function deploy_dvn {
    dfx deploy dvn 2>&1
    dfx canister call dvn init_providers 2>&1
}

function get_dvn_address {
    dfx canister call dvn public_key | awk -F'"' '{ print $2 }'
}
