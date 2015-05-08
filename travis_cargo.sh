#!/bin/sh

FEATURES=

if [ "$TRAVIS_RUST_VERSION" = "nightly" ]; then
    FEATURES="--features unstable"
else
    if [ "$1" = "bench" ]; then exit 0; fi
fi

exec cargo "$@" $FEATURES
