#! /bin/sh

cross build --release --target aarch64-unknown-linux-gnu

scp target/aarch64-unknown-linux-gnu/release/magic-mirrot-switch mirror@erised:
