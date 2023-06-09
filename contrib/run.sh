#!/bin/sh

set -e

cargo run -- --crate-name "$1" "../programs/$1.cl"
echo

echo "Compiling LLVM IR..."
llc --filetype=obj -o ../programs/bin/main.o ../programs/bin/main.ll

echo "Linking program..."
gcc -std=c17 -lm -o ../programs/bin/main ../programs/bin/main.o

echo
../programs/bin/main
