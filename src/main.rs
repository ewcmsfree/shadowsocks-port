use shadowsocks_port;
use shadowsocks_port::context::{Context, ShadowsocksPort};
use shadowsocks_port::immortalwrt::Immortalwrt;
use shadowsocks_port::macos::MacOS;
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
    let shadowsocks_path = config.shadowsocks.get_path();
    let path = Path::new(&shadowsocks_path);

    let os = config.shadowsocks.get_os().to_lowercase();

    let mut os_context: Box<dyn ShadowsocksPort> = Box::new(Immortalwrt);
    if os.contains("macos") {
        os_context = Box::new(MacOS);
    } else if os.contains("windows") {
        todo!()
    } else if os.contains("linux")
        || os.contains("ubuntu")
        || os.contains("debian")
        || os.contains("centos")
    {
        todo!()
    }
    let context = Context::new(os_context);

    loop {
        let mut port: u32 = context.read_shadowsocks_port(path)?;
        let content = read_file_async(&remote_file)
            .await?
            .replace("\n", "")
            .replace("\r", "");
        if content.parse::<u32>().is_ok() && port != content.parse::<u32>()? {
            port = content.parse::<u32>()?;
            let _ = context.modify_shadowsocks_port(path, port);
            // Command::new("reboot").status()?;
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
