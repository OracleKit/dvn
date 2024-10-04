_LOGS_DIR=./logs

function _get_logs_file_path {
    echo "$_LOGS_DIR/$1.txt"
}

# Usage: [COMPONENT_NAME]
function pretty_log {
    PREFIX=$1
    LOG_FILE=$(_get_logs_file_path $1)
    while IFS= read -r line; do
        echo $line >> $LOG_FILE
        echo "[" $PREFIX "]" $line
    done
}

function clear_logs {
    rm -rf $_LOGS_DIR 2>&1
    mkdir $_LOGS_DIR
}