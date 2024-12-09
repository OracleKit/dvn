source $DIRNAME/log.bash

export SINK_DIR=./.sink
export SINK_LOGS_DIR=$SINK_DIR/logs
export SINK_ENV_DIR=$SINK_DIR/env
export SINK_BIN_DIR=$SINK_DIR/bin
export SUITE_CHAINS_LIST_FILE=$SINK_ENV_DIR/chains
export SUITE_GENERATED_ENV_FILE=$SINK_ENV_DIR/.env.generated
export USER_ENV_FILE=.env.local

function _terminate_trap {
    exit_code=$?
    set +e
    jobs -p | xargs kill -s SIGTERM 2>/dev/null
    echo "Terminating..." | pretty_log_term bash
    exit $exit_code
}

function terminate {
    _terminate_trap
    exit
}

function setup_trap_handlers {
    trap _terminate_trap SIGINT
    trap _terminate_trap SIGTERM
    trap _terminate_trap EXIT
}

function setup_directories {
    set +e
    mkdir $SINK_DIR 2>/dev/null
    mkdir $SINK_LOGS_DIR 2>/dev/null
    mkdir $SINK_ENV_DIR 2>/dev/null
    mkdir $SINK_BIN_DIR 2>/dev/null
    touch $SUITE_CHAINS_LIST_FILE 2>/dev/null
    touch $SUITE_GENERATED_ENV_FILE 2>/dev/null
    set -e
}

# Usage: [ENV_FILE_NAME]
function source_env {
    set -a
    source $1
    set +a
}

# Usage: [ENV_FILE_NAME]
function renew_file {
    rm $1 2>/dev/null || true
    touch $1
}