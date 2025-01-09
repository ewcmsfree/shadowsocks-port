#!/bin/bash -e

## 设置参数值
TARGET=x86_64-unknown-linux-musl

## 添加 $TARGET 到 toolchain
rustup target add $TARGET

#export CC_X86_64_UNKNOWN_LINUX_MUSL=x86_64-unknown-linux-musl-gcc
#export CXX_X86_64_UNKNOWN_LINUX_MUSL=x86_64-unknown-linux-musl-g++
#export AR_X86_64_UNKNOWN_LINUX_MUSL=x86_64-unknown-linux-musl-ar
#export CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=x86_64-unknown-linux-musl-gcc

export CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=x86_64-linux-musl-gcc

## 构建项目工程
cargo build --release --target $TARGET
## 构建项目工程并打印生成文件的大小
#cargo size --release --target $TARGET -- -A
## 构建项目工程并打印出成文件中各引用包占用的大小
#cargo bloat --release --target $TARGET