source $DIRNAME/log.bash

LOG_FILE_DFX=$(pretty_log_file dfx)

function _pretty_log_term_dfx {
    pretty_log_term dfx
}

function _wait_for_dfx_start {
    until grep "Dashboard" $LOG_FILE_DFX >/dev/null 2>&1; do
        sleep .1
    done
}

function start_dfx {
    echo "Starting local network..." | _pretty_log_term_dfx
    dfx stop >/dev/null 2>&1
    dfx start --clean >$LOG_FILE_DFX 2>&1 &
    
    _wait_for_dfx_start
    tail -n0 -f $LOG_FILE_DFX | _pretty_log_term_dfx &
    echo "Local network started" | _pretty_log_term_dfx
}

function deploy_dvn {
    dfx deploy dvn 2>&1
    dfx canister call dvn init_dvn 2>&1
}

function get_dvn_address {
    dfx canister call dvn address | awk -F'"' '{ print $2 }'
}
