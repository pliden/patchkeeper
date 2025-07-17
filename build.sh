#!/bin/sh

set -x
cargo build $CARGO_BUILD_OPTIONS "$@"
cargo clippy -- -D warnings

# End of file
