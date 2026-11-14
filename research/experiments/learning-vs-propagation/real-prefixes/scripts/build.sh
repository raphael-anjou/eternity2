#!/bin/sh
# learning-real-prefixes: build the three C programs into bin/. clasp 3.3.10 (commit 3f3db1e, cmake Release, default
# options) is built separately from the same source on every machine and copied to bin/clasp.
# usage: sh build.sh        (on a machine without a system compiler: any shell that provides gcc)
cd "$(dirname "$0")"; mkdir -p bin
cc -O2 -o bin/mkprefixes mkprefixes.c && cc -O2 -o bin/rowscan_real rowscan_real.c && cc -O2 -DMAXC=256 -DW=16 -o bin/dyn16 dyn.c -lm && echo built
