#!/bin/bash
cargo build --release
cargo build --release --target x86_64-pc-windows-gnu
cp ./target/release/ynab-converter ./binaries/linux/ynab-converter
cp ./target/x86_64-pc-windows-gnu/release/ynab-converter.exe ./binaries/windows/ynab-converter.exe