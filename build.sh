#!/bin/sh

set -ex
cargo build $CARGO_BUILD_OPTIONS "$@"
cargo clippy -- --deny warnings
cargo fmt --check --verbose

# End of file
