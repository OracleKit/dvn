source $DIRNAME/log.bash

SSL_LOG_FILE=$(pretty_log_file ssl)

function ssl_pretty_log_term {
    pretty_log_term ssl
}


# Usage: [APP_PORT] [SSL_PORT]
function ssl_start_proxy {
    npx local-ssl-proxy \
        --source $2 --target $1 \
        --cert localhost.pem --key localhost-key.pem \
        >$SSL_LOG_FILE 2>&1 &
    
    while grep "Started proxy: https://localhost:$2" $SSL_LOG_FILE >/dev/null 2>&1; do
        sleep .1
    done
    echo "Started ssl proxy for port $1" | ssl_pretty_log_term
}
