if [ ! "$TRAVIS_RUST_VERSION" = "nightly" ]; then
    cp Cargo-stable.toml Cargo.toml
    rm -r benches
fi
