source $DIRNAME/log.bash

LOG_FILE_SSL=$(pretty_log_file ssl)

function _pretty_log_term_ssl {
    pretty_log_term ssl
}


# Usage: [APP_PORT] [SSL_PORT]
function start_ssl_proxy {
    npx local-ssl-proxy \
        --source $2 --target $1 \
        --cert localhost.pem --key localhost-key.pem \
        >$LOG_FILE_SSL 2>&1 &
    
    while grep "Started proxy: https://localhost:$2" $LOG_FILE_SSL >/dev/null 2>&1; do
        sleep .1
    done
    echo "Started ssl proxy for port $1" | _pretty_log_term_ssl
}
