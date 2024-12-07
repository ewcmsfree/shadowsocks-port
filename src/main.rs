use shadowsocks_port;
use shadowsocks_port::context::{Context, ShadowsocksPort};
use shadowsocks_port::immortalwrt::Immortalwrt;
use shadowsocks_port::log::set_tracing_subscriber;
use shadowsocks_port::macos::MacOS;
use shadowsocks_port::windows::Windows;
use shadowsocks_port::{config::Config, remote::read_file_async};
use std::path::Path;
use std::process::Command;
use tracing::{debug, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 读取 shadowsocks-config.yml 的配置文件
    let filename = Config::read_default_config()?;
    let config = Config::load(filename)?;

    set_tracing_subscriber(&config);

    // 读取远程配置文件内容
    let remote_file = config.remote_file.connect_remote_file_url();
    debug!("remote file: {remote_file}");

    let shadowsocks_config = config.shadowsocks.get_config();
    debug!("local shadowsocks config: {shadowsocks_config}");
    let shadowsocks_os = config.shadowsocks.get_os().to_lowercase();
    debug!("local shadowsocks os: {shadowsocks_os}");
    let shadowsocks_command = config.shadowsocks.get_command();
    debug!("local shadowsocks command: {shadowsocks_command}");

    let shadowsocks_config_path = Path::new(&shadowsocks_config);
    let mut os_context: Box<dyn ShadowsocksPort> = Box::new(Immortalwrt);
    if shadowsocks_os.contains("macos") {
        os_context = Box::new(MacOS);
    } else if shadowsocks_os.contains("windows") {
        os_context = Box::new(Windows);
    }
    let context = Context::new(os_context);

    loop {
        info!("start polling");
        let remote_port = read_file_async(&remote_file)
            .await?
            .replace("\n", "")
            .replace("\r", "");
        let remote_port: u32 = remote_port.parse()?;
        debug!("remote port: {remote_port}");

        let local_port: u32 = context
            .read_shadowsocks_port(shadowsocks_config_path)
            .await?;
        debug!("local shadowsocks port: {local_port}");

        if local_port != remote_port {
            info!("changed port");
            let _ = context
                .modify_shadowsocks_port(shadowsocks_config_path, remote_port)
                .await?;
            debug!("modify local shadowsocks port success");
            if !shadowsocks_command.is_empty() {
                info!("local command restarting");
                Command::new(&shadowsocks_command).status()?;
            }
        } else {
            debug!("no need to change port");
        }
        info!("end pool");
        tokio::time::sleep(std::time::Duration::from_secs(37)).await;
    }
}
