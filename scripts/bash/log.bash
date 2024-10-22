_LOGS_DIR=./.sink/logs

# Usage: [COMPONENT_NAME]
function pretty_log_file {
    echo "$_LOGS_DIR/$1.txt"
}

# Usage: [COMPONENT_NAME]
function pretty_log_term {
    PREFIX=$1
    while IFS=$(echo -en "\n\b") read -r line; do
        echo "[" $PREFIX "]" $line
    done
}

# Usage: [COMPONENT_NAME]
function pretty_log {
    PREFIX=$1
    LOG_FILE=$(pretty_log_file $1)
    while IFS= read -r line; do
        echo $line >> $LOG_FILE
        echo $line | pretty_log_term $PREFIX
    done
}

function clear_logs {
    rm -rf $_LOGS_DIR 2>&1
    mkdir $_LOGS_DIR
}