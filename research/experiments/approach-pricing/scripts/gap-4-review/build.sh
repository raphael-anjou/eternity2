#!/bin/sh
# builds one binary per board-size class
cd "$(dirname "$0")"
cc -O3 -mcpu=native -DMAXC=64 -DW=4 -o dyn8 dyn.c -lm
cc -O3 -mcpu=native -DMAXC=144 -DW=9 -o dyn12 dyn.c -lm
cc -O3 -mcpu=native -DMAXC=256 -DW=16 -o dyn16 dyn.c -lm
cp dyn16 dyn16p   # same binary; the profile runs used a separate copy so as not to overwrite a running one
