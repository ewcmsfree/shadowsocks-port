use shadowsocks_port;
use shadowsocks_port::context::{Context, ShadowsocksPort};
use shadowsocks_port::immortalwrt::Immortalwrt;
use shadowsocks_port::macos_and_windows::MacOSAndWindows;
use shadowsocks_port::{config::Config, remote::read_file_async};
use std::path::Path;
use std::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 读取 shadowsocks-config.yml 的配置文件
    let filename = Config::read_default_config()?;
    let config = Config::load(filename)?;

    // 读取远程配置文件
    let remote_file = config.remote_file.connect_remote_file_url();
    let shadowsocks_config = config.shadowsocks.get_config();
    let shadowsocks_os = config.shadowsocks.get_os().to_lowercase();
    let shadowsocks_command = config.shadowsocks.get_command();

    let shadowsocks_config_path = Path::new(&shadowsocks_config);
    let mut os_context: Box<dyn ShadowsocksPort> = Box::new(Immortalwrt);
    if shadowsocks_os.contains("macos") || shadowsocks_os.contains("windows") {
        os_context = Box::new(MacOSAndWindows);
    }
    let context = Context::new(os_context);

    loop {
        println!("start new loop");
        let mut port: u32 = context
            .read_shadowsocks_port(shadowsocks_config_path)
            .await?;
        let content = read_file_async(&remote_file)
            .await?
            .replace("\n", "")
            .replace("\r", "");
        if content.parse::<u32>().is_ok() && port != content.parse::<u32>()? {
            println!("port changed...");
            port = content.parse::<u32>()?;
            let _ = context
                .modify_shadowsocks_port(shadowsocks_config_path, port)
                .await?;
            if !shadowsocks_command.is_empty() {
                println!("command restarting...");
                Command::new(&shadowsocks_command).status()?;
            }
        }
        tokio::time::sleep(std::time::Duration::from_secs(37)).await;
    }
}
