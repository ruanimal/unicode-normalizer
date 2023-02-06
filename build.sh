#!/bin/sh

set -ex

mkdir -p dist
cargo build --release --target=x86_64-pc-windows-gnu
zip -j -r dist/x86_64-pc-windows-gnu.zip target/x86_64-pc-windows-gnu/release/unicode-normalizer.exe
cargo build --release --target=x86_64-apple-darwin
zip -j -r dist/x86_64-apple-darwin.zip target/x86_64-apple-darwin/release/unicode-normalizer
