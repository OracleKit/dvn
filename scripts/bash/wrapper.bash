set -e
set -o pipefail

DIRNAME=$(dirname $0)
source $DIRNAME/eth.bash
source $DIRNAME/dfx.bash
source $DIRNAME/ssl.bash
source $DIRNAME/log.bash
source $DIRNAME/common.bash

setup_trap_handlers

# Setup .sink 
setup_directories

# Load user env
if [ -f $USER_ENV_FILE ]; then
    source_env $USER_ENV_FILE
else
    echo "Didn't find env file"
    terminate
fi

if [ -f $SUITE_GENERATED_ENV_FILE ]; then
    source_env $SUITE_GENERATED_ENV_FILE
fi

script_to_run=$1
shift 1
source $script_to_run