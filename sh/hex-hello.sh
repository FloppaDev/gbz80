#!/bin/sh

dir=$(dirname "$0")
hello="$dir/../build/hello.gb"

xxd -g 1 $hello | less
