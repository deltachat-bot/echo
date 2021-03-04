#!/bin/bash
set -e

export PKG_CONFIG_PATH=/usr/local/lib/pkgconfig
gcc main.c -o main $(pkg-config --cflags --libs deltachat) -lpthread
LD_LIBRARY_PATH="/usr/local/lib" ./main
