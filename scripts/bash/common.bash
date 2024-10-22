source $DIRNAME/log.bash

export SINK_DIR=./.sink
export SINK_LOGS_DIR=$SINK_DIR/logs
export SINK_ENV_DIR=$SINK_DIR/env
export SUITE_CHAINS_LIST_FILE=$SINK_ENV_DIR/chains
export SUITE_CHAINS_ENV_FILE=$SINK_ENV_DIR/.env.chains
export SUITE_GENERATED_ENV_FILE=$SINK_ENV_DIR/.env.generated
export USER_CHAINS_ENV_FILE=.env.chains
export USER_CONFIG_ENV_FILE=.env.config
export DEPLOYED_ENV_LOCAL_FILE=.env.local

function _terminate_trap {
    jobs -p | xargs kill -s SIGTERM 2>/dev/null
    echo "Terminating..." | pretty_log_term bash
    exit
}

function terminate {
    _terminate_trap
    exit
}

function setup_trap_handlers {
    trap _terminate_trap SIGINT
    trap _terminate_trap SIGTERM
}

function setup_directories {
    mkdir $SINK_DIR 2>/dev/null
    mkdir $SINK_LOGS_DIR 2>/dev/null
    mkdir $SINK_ENV_DIR 2>/dev/null
    touch $SUITE_CHAINS_LIST_FILE 2>/dev/null
    touch $SUITE_CHAINS_ENV_FILE 2>/dev/null
    touch $SUITE_GENERATED_ENV_FILE 2>/dev/null
}

# Usage: [ENV_FILE_NAME]
function source_env {
    export $(cat $1 | xargs)
}

# Usage: [ENV_FILE_NAME]
function renew_file {
    rm $1 2>/dev/null
    touch $1
}