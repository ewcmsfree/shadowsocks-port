#!/bin/bash -e

## 设置参数值
TARGET=x86_64-pc-windows-gnu

## 添加 $TARGET 到 toolchain
rustup target add $TARGET

## 运行 cargo 清除
#cargo clean

## 构建项目工程
#cargo build --release --target $TARGET
## 构建项目工程并打印生成文件的大小
#cargo size --release --target $TARGET
## 构建项目工程并打印出成文件中各引用包占用的大小
cargo bloat --release --target $TARGET