#!/usr/bin/env bash

set -eE

cargo build --release

for EXT in lib dll a so; do
    if [ -f "./target/release/libcomplex_polynomials.${EXT}" ]; then
        cp "./target/release/libcomplex_polynomials.${EXT}" ../c_caller/lib/
    fi
done

