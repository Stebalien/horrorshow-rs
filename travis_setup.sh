gawk -i inplace '!(/# target: / && !/\<'$TRAVIS_RUST_VERSION'\>/) { print($0); }' Cargo.toml
if [ ! "$TRAVIS_RUST_VERSION" = "nightly" ]; then
    rm -r benches
fi
