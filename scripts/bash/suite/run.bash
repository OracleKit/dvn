set -e

DIRNAME=$(dirname $(dirname $0))
source $DIRNAME/common.bash

source_env $SUITE_CHAINS_ENV_FILE
source_env $USER_CONFIG_ENV_FILE
source_env $SUITE_GENERATED_ENV_FILE

npx tsx ./test/scripts/$1