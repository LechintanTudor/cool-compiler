#!/bin/sh

cargo run -- --crate-name test ../programs/main.cl
llc --filetype=obj -o ../programs/bin/main.o ../programs/bin/main.ll
gcc -std=c17 -lm -o ../programs/bin/main ../programs/bin/main.o

echo
../programs/bin/main
