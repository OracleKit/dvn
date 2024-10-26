set -e

DIRNAME=$(dirname $(dirname $0))
source $DIRNAME/common.bash

chain=$(echo $1 | tr '[:lower:]' '[:upper:]')
if grep $chain $USER_CHAINS_ENV_FILE >/dev/null 2>&1; then
    if grep $1 $SUITE_CHAINS_LIST_FILE >/dev/null 2>&1; then
        echo "Failed: Chain already exists in suite: $1"
        exit 1
    else
        echo $1 >> $SUITE_CHAINS_LIST_FILE
        grep $chain $USER_CHAINS_ENV_FILE >> $SUITE_CHAINS_ENV_FILE
        echo "Success: $1 added to suite"
    fi
else
    echo "Failed: Invalid chain name: $1"
    exit 1
fi
