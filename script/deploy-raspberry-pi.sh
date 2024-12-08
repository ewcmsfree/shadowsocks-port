#!/bin/bash -e

# 设置参数值
USER=root
PI_IP=192.168.0.99
TARGET=aarch64-unknown-linux-musl

# 添加 $TARGET 到 toolchain
rustup target add $TARGET

## 构建项目工程
#cargo build --release --target $TARGET
## 构建项目工程并打印生成文件的大小
#cargo size --release --target $TARGET
## 构建项目工程并打印出成文件中各引用包占用的大小
cargo bloat --release --target $TARGET

# 拷贝文件到远程树莓派
scp -r ../target/$TARGET/release/shadowsocks-port $USER@$PI_IP:/usr/bin/

#REMOTE_SERVER="$USER@$PI_IP"
#COMMAND="/usr/bin/shadowsocks-port"
#ssh $REMOTE_SERVER "$COMMAND &"