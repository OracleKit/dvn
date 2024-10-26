set -e

DIRNAME=$(dirname $(dirname $0))
source $DIRNAME/common.bash

rm -r $SINK_DIR 2>/dev/null || true
rm -r $DEPLOYED_ENV_LOCAL_FILE 2>/dev/null || true
setup_directories
