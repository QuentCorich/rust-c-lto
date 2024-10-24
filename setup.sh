#!/bin/bash
export PATH=/usr/lib/llvm-18/bin:$PATH
export RUSTFLAGS="-Clinker-plugin-lto -Copt-level=2 -Clinker=clang -Clink-arg=-fuse-ld=lld"