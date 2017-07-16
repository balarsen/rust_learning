#!/usr/bin/env python

# cargo build --release

from ctypes import cdll

lib = cdll.LoadLibrary("target/release/libembed.dylib")

lib.process()

print("done!")
