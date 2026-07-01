#!/bin/sh
#exec cargo build RUSTFLAGS="--cfg wasmedge --cfg tokio_unstable" --target wasm32-wasip1 --release $*

exec cargo build --release $*

