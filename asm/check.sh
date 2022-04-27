#!/bin/sh

a="\033[34;1m"
b="\033[0;1m"
c="\033[0m"
build="$a------------------------------------- BUILD ------------------------------------\n\n"
close="$a--------------------------------------------------------------------------------\n\n$c"

r() {
    file=$1
    printf "$build$b        $file:\n\n$close"
    cargo r -- $file -o tmp 2>&1 >/dev/null
}

r asm/hello/hello.gb.asm
r asm/tests/main.gb.asm
r asm/tests/expr.gb.asm

rm tmp 2>/dev/null
