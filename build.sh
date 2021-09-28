#!/bin/bash
cargo fmt
wasm-pack build -t no-modules
