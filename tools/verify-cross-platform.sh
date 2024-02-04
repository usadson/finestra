#!/bin/bash
set -e

on_exit () {
    echo "\x1b[31;49;1mFAILURE\033[0m"
}

trap on_exit ERR

cargo build --target x86_64-pc-windows-gnu || exit
cargo test || exit

echo "\x1b[32;49;1mSUCCESS\033[0m"
