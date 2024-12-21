chain=$(echo $1 | tr '[:lower:]' '[:upper:]')
if grep "${chain}_" $USER_ENV_FILE >/dev/null 2>&1; then
    if grep -i $1 $SUITE_CHAINS_LIST_FILE >/dev/null 2>&1; then
        echo "Failed: Chain already exists in suite: $1"
        exit 1
    else
        echo $1 >> $SUITE_CHAINS_LIST_FILE
        echo "Success: $1 added to suite"
    fi
else
    echo "Failed: Invalid chain name: $1"
    exit 1
fi
