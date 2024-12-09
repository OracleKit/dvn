source $DIRNAME/log.bash
source $DIRNAME/common.bash

DFX_LOG_FILE=$(pretty_log_file dfx)

function dfx_pretty_log_term {
    pretty_log_term dfx
}

function dfx_wait_for_start {
    until grep "Dashboard" $DFX_LOG_FILE >/dev/null 2>&1; do
        sleep .1
    done
}

# Usage: [PORT]
function dfx_start {
    port=$1
    echo "Starting local network..." | dfx_pretty_log_term
    dfx stop >/dev/null 2>&1
    dfx start --clean >$DFX_LOG_FILE --host "0.0.0.0:$port" 2>&1 &
    
    dfx_wait_for_start
    tail -n0 -f $DFX_LOG_FILE | dfx_pretty_log_term &
    echo "Local network started" | dfx_pretty_log_term
}

function dfx_deploy_dvn {
    dfx deploy dvn 2>&1
    dfx canister call dvn init 2>&1
}

function dfx_get_dvn_address {
    dfx canister call dvn address | awk -F'"' '{ print $2 }'
}

# Usage: [RPC_URL] [CHAIN_ID] [ENDPOINT_ID] [DVN_ADDRESS]
function dfx_add_dvn_chain {
    dfx canister call dvn add_chain "(\"$1\", $2, $3, \"$4\")" 2>&1
}

# Installs to $SINK_BIN_DIR/pocket-ic
function dfx_setup_pocketic_bin {
    original_dir=$(pwd)
    cd $SINK_BIN_DIR
    rm -r $SINK_BIN_DIR/pocket-ic 2>/dev/null || true
    
    if [ "$(uname)" == "Darwin" ]; then
        wget https://github.com/dfinity/pocketic/releases/download/7.0.0/pocket-ic-x86_64-darwin.gz
        gzip -d pocket-ic-x86_64-darwin.gz
        rm pocket-ic-x86_64-darwin.gz 2>/dev/null || true
        mv pocket-ic-x86_64-darwin pocket-ic
        chmod +x pocket-ic
        xattr -dr com.apple.quarantine pocket-ic
    else
        wget https://github.com/dfinity/pocketic/releases/download/7.0.0/pocket-ic-x86_64-linux.gz
        gzip -d pocket-ic-x86_64-linux.gz
        rm pocket-ic-x86_64-linux.gz 2>/dev/null || true
        mv pocket-ic-x86_64-linux pocket-ic
        chmod +x pocket-ic
    fi

    cd $original_dir
}
