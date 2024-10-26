set -e

DIRNAME=$(dirname $(dirname $0))
source $DIRNAME/common.bash

echo
echo "Chains in suite:"
cat $SUITE_CHAINS_LIST_FILE
echo
