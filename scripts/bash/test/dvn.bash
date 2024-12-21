export POCKET_IC_BIN=$(pwd)/.sink/bin/pocket-ic
RUSTFLAGS="$RUSTFLAGS -A dead_code -A unused_imports" cargo test