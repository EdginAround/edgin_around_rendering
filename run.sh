#!/bin/bash

function usage() {
    echo 'Commands:'
    echo ' - setup - installs `cargo` toolchains using `rustup`'
    echo ' - build_debug - builds debug version of the library'
    echo ' - build_android - builds release version of the library for four Android platforms'
    echo ' - build or build_all - builds everything'
    echo ' - tests - runs unit tests'
    echo ' - format - runs `rust-fmt` code formatter'
    echo ' - clean - removes all build artifacts'
}

function run_setup() {
    rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android
}

function run_build_debug() {
    cargo build --all
}

function run_build_android() {
    cargo build -p edgin_around_android --target aarch64-linux-android --release
    cargo build -p edgin_around_android --target armv7-linux-androideabi --release
    cargo build -p edgin_around_android --target i686-linux-android --release
    cargo build -p edgin_around_android --target x86_64-linux-android --release
}

function run_tests() {
    cargo test --all
}

function run_format() {
    cargo fmt --all
}

function run_clean() {
    rm -rf target
}

if (( $# > 0 )); then
    command=$1
    shift

    case $command in
        'setup')
            run_setup
            ;;
        'build_debug')
            run_build_debug
            ;;
        'build_android')
            run_build_android
            ;;
        'build_all'|'build')
            run_build_debug
            run_build_android
            ;;
        'tests')
            run_tests
            ;;
        'format')
            run_format
            ;;
        'clean')
            run_clean
            ;;
        *)
            echo "Command \"$command\" unknown."
            echo
            usage
            ;;
    esac
else
    echo 'Please give a command.'
    echo
    usage
fi

