#!/bin/bash -e

# 设置参数值
TARGET=x86_64-pc-windows-gnu

# 添加 $TARGET 到 toolchain
rustup target add $TARGET

# 运行 cargo 构建
cargo clean
cargo build --release --target $TARGET
