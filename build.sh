#!/bin/bash
# SPDX-License-Identifier: Apache-2.0
cargo fmt
wasm-pack build -t no-modules
rm -rf public/pkg
mv pkg public
