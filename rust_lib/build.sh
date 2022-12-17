#!/usr/bin/env bash

set -eE

cargo build --release
cp ./target/release/libcomplex_polynomials.lib ../c_caller/lib/
cp ./target/release/libcomplex_polynomials.dll ../c_caller/lib/

