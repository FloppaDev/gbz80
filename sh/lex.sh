#!/bin/sh

pushd $(pwd)
cd gen/lex
cargo r
popd
