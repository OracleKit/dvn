set -e

DIRNAME=$(dirname $(dirname $0))
source $DIRNAME/common.bash

source_env $USER_ENV_FILE
source_env $SUITE_GENERATED_ENV_FILE

npx tsx ./scripts/ts/deploy.ts $(cat $SUITE_CHAINS_LIST_FILE | xargs)