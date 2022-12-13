#!/bin/sh

grep -rnH --color=always --exclude-dir={.git,target,sh,.vim} "$1" ./
