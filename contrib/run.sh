#!/bin/sh

set -e

cargo run -- --crate-name test ../programs/main.cl
echo

echo "Compiling LLVM IR..."
llc --filetype=obj -o ../programs/bin/main.o ../programs/bin/main.ll

echo "Linking program..."
gcc -std=c17 -lm -o ../programs/bin/main ../programs/bin/main.o

echo
../programs/bin/main
