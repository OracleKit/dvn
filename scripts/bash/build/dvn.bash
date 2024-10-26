set -e

export RUSTFLAGS="--cfg profile=\"ephemeral-build\""
dfx build --check dvn
dfx generate dvn