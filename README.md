1. 首先远程登录到远程的树莓派后，再回到本机执行 script/copy-ssh-keys.sh 命令，使本机无密码 SSH 到远程的树莓派

2. 使用 scp 命令，出现 /usr/libexec/sftp-server: not found 的错误时，需要在 ImmortalWrt 上安装 openssh-sftp-server 的软件包
    ```shell
    $ opkg update
    $ opkg install openssh-sftp-server
    ```

3. 使用 script/deploy.sh 命令，出现：rust error: linking with `cc` failed: exit status: 1 的错误
    ```shell
    $ brew install FiloSottile/musl-cross/musl-cross
    $ rustup target add aarch64-unknown-linux-musl
    ```
    在 ~/.cargo 目录下 config.toml 文件中加入
    ```toml
    [target.aarch64-unknown-linux-musl]
    linker = "aarch64-linux-musl-gcc"
    rustflags = ["-C", "link-arg=-static"]
    ```
    然后再运行 deploy.sh 命令
    
    说明：对于 aarch64-unknown-linux-musl 中的 aarch64 这是根据机器的 CPU 架构来确定的，可以通过 `uname -m` 命令来查看， 如果是 Inter 的 CPU，则使用 x86_64-unknown-linux-musl

4. 把 script/shadowsocks-port.sh 脚本放到服务器的 /etc/init.d 目录下，然后执行以下命令
   ```shell
   $ chmod +x /etc/init.d/shadowsocks-port.sh
   $ /etc/init.d/shadowsocks-port.sh enable
   $ /etc/init.d/shadowsocks-port.sh start
   ```
   把 fixtures/shadowsocks-config.yml 文件放到服务器的 /etc 目录下（自启动情况下）
5. Rustc/LLVM Target Triple
   <arch><sub>-<vendor>-<sys>-<env>
   - arch = x86_64，i386，arm，...
   - sub = [ex. arm] v5，v6，v7m，...
   - vendor = [optional] pc，apple，ibm，...
   - sys = none，linux，windows，darwin，...
   - env = eabi，gnu，elf，...