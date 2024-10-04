# Usage: [APP_PORT] [SSL_PORT]
function start_ssl_proxy {
    npx local-ssl-proxy \
        --source $2 --target $1 \
        --cert localhost.pem --key localhost-key.pem \
        2>&1
}
