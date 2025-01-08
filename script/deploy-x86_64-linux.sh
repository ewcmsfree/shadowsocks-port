#!/bin/bash -e

## 设置参数值
TARGET=x86_64-unknown-linux-gnu

## 添加 $TARGET 到 toolchain
rustup target add $TARGET

## 运行 cargo 清除
cargo clean

#export CC_X86_64_UNKNOWN_LINUX_GNU=x86_64-unknown-linux-gnu-gcc
#export CXX_X86_64_UNKNOWN_LINUX_GNU=x86_64-unknown-linux-gnu-g++
#export AR_X86_64_UNKNOWN_LINUX_GNU=x86_64-unknown-linux-gnu-ar
#export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-unknown-linux-gnu-gcc

export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-linux-gnu-gcc

## 构建项目工程
#cargo build --release --target $TARGET
## 构建项目工程并打印生成文件的大小
#cargo size --release --target $TARGET -- -A
## 构建项目工程并打印出成文件中各引用包占用的大小
cargo bloat --release --target $TARGET