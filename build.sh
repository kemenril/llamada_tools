#!/bin/sh
exec cargo build --target wasm32-wasip1 --release $*

