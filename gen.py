import argparse, subprocess;

bindgen = "bindgen"

harfbuzz = "../harfbuzz/src/hb.h"
includes = [
    "-I", "../harfbuzz/src",
    "-I", "."
    ]
sysincludes = [
    "-isystem", "/usr/lib/x86_64-linux-gnu/gcc/x86_64-linux-gnu/4.5/include",
    "-isystem", "/usr/lib/gcc/x86_64-linux-gnu/4.6/include"
    ]
otherflags = [
    ]

args = [
    bindgen,
    "-l", "harfbuzz",
    "-o", "harfbuzz.rs",
    "-match", "hb",
    harfbuzz]
args += includes + sysincludes + otherflags

subprocess.call(args)

        
