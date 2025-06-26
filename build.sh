#!/bin/sh

cargo clippy -- -D warnings
cargo build $CARGO_BUILD_OPTIONS "$@"

# End of file
