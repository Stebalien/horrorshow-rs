#!/bin/sh

ARGS=

if [ "$TRAVIS_RUST_VERSION" = "nightly" ]; then
    ARGS="--features unstable"
else
    if [ "$1" = "bench" ]; then exit 0; fi
    cp Cargo-stable.toml Cargo.toml
fi

exec cargo "$@" $ARGS
