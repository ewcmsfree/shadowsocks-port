use shadowsocks_port;
use shadowsocks_port::context::{Context, ShadowsocksPort};
use shadowsocks_port::immortalwrt::Immortalwrt;
use shadowsocks_port::log::set_tracing_subscriber;
use shadowsocks_port::macos::MacOS;
use shadowsocks_port::windows::Windows;
use shadowsocks_port::{config::Config, remote::read_file_async};
use std::path::Path;
use std::process::Command;
use tracing::{debug, error, info};

#[tokio::main]
async fn main() -> ! {
    // 读取 shadowsocks-config.yml 的配置文件，读取不到直接退出程序
    let filename = Config::read_default_config().unwrap();
    let config = Config::load(filename).unwrap();

    set_tracing_subscriber(config.level.get_log_level());
    info!("shadowsocks-port start");

    // 读取远程配置文件内容
    let remote_file = config.remote_file.connect_remote_file_url();
    debug!("remote file: {remote_file}");

    let shadowsocks_config = config.shadowsocks.get_config();
    debug!("local shadowsocks config: {shadowsocks_config}");
    let shadowsocks_os = config.shadowsocks.get_os().to_lowercase();
    debug!("local shadowsocks os: {shadowsocks_os}");
    let shadowsocks_command = config.shadowsocks.get_command();
    debug!("local shadowsocks command: {shadowsocks_command}");

    let mut os_context: Box<dyn ShadowsocksPort + Send> = Box::new(Immortalwrt);
    if shadowsocks_os.contains("macos") {
        os_context = Box::new(MacOS);
    } else if shadowsocks_os.contains("windows") {
        os_context = Box::new(Windows);
    }
    let context = Context::new(os_context);

    // 创建一个新的异步任务来修改端口号
    tokio::spawn(async move {
        modify_config(
            context.shadowsocks_port,
            remote_file,
            shadowsocks_config,
            shadowsocks_command,
        )
        .await;
    });

    // 主程序一直执行
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
    }
}

async fn modify_config(
    shadowsocks_port: Box<dyn ShadowsocksPort + Send>,
    remote_file: String,
    shadowsocks_config: String,
    shadowsocks_command: String,
) {
    let shadowsocks_config_path = Path::new(&shadowsocks_config);
    loop {
        info!("start polling");

        let remote_port = match read_file_async(&remote_file).await {
            Ok(remote_port) => remote_port.replace("\n", "").replace("\r", ""),
            Err(err) => {
                error!("read remote file error: {err}");
                continue;
            }
        };
        let remote_port: u32 = match remote_port.parse() {
            Ok(remote_port) => remote_port,
            Err(err) => {
                error!("parse remote port error: {err}");
                continue;
            }
        };
        debug!("remote port: {remote_port}");

        let local_port: u32 = match shadowsocks_port
            .read_shadowsocks_port(shadowsocks_config_path)
            .await
        {
            Ok(local_port) => local_port,
            Err(err) => {
                error!("read local shadowsocks port error: {err}");
                continue;
            }
        };
        debug!("local shadowsocks port: {local_port}");

        if local_port != remote_port {
            info!("changed port");
            match shadowsocks_port
                .modify_shadowsocks_port(shadowsocks_config_path, remote_port)
                .await
            {
                Ok(_) => {
                    info!("modify local shadowsocks port success");
                }
                Err(err) => {
                    error!("modify local shadowsocks port error: {err}");
                    continue;
                }
            };
            if !shadowsocks_command.is_empty() {
                info!("local command restarting");
                match Command::new(&shadowsocks_command).status() {
                    Ok(_) => {
                        info!("local command success");
                    }
                    Err(err) => {
                        error!("local command error: {err}");
                        continue;
                    }
                };
            }
        } else {
            debug!("no need to change port");
        }
        info!("end poll");
        tokio::time::sleep(std::time::Duration::from_secs(37)).await;
    }
}
