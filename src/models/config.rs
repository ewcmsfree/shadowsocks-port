use std::fs;
use std::path::Path;

/// 配置信息
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub remote_file: RemoteFileConfig,  // 远程服务文件
    pub shadowsocks: ShadowsocksConfig, // shadowsocks配置
}

impl Config {
    /// 读取配置文件
    pub fn load(filename: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let config = fs::read_to_string(filename)?;
        let config = serde_yaml::from_str(&config)?;
        Ok(config)
    }

    /// 读取默认的配置
    pub fn read_default_config() -> anyhow::Result<String, String> {
        let filename = std::env::var("SHADOWSOCKS_CONFIG").unwrap_or_else(|_| {
            let first_path = Path::new("./shadowsocks-config.yml");
            let path = shellexpand::tilde("~/.config/shadowsocks-config.yml");
            let second_path = Path::new(path.as_ref());
            let third_path = Path::new("/etc/shadowsocks-config.yml");
            match (
                first_path.exists(),
                second_path.exists(),
                third_path.exists(),
            ) {
                (true, _, _) => first_path.to_str().unwrap().to_string(),
                (_, true, _) => second_path.to_str().unwrap().to_string(),
                (_, _, true) => third_path.to_str().unwrap().to_string(),
                _ => panic!("no config file found"),
            }
        });
        Ok(filename)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RemoteFileConfig {
    pub host: String, // 服务器地址
    pub port: u32,    // 服务器端口
    pub file: String, // 服务器文件
    pub ssl: bool,    // 是否使用ssl
}

impl RemoteFileConfig {
    pub fn connect_remote_file_url(&self) -> String {
        format!(
            "http{}://{}:{}/{}",
            if self.ssl { "s" } else { "" },
            self.host,
            self.port,
            self.file
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ShadowsocksConfig {
    pub os: String,      // 操作系统
    pub config: String,  // 配置文件路径
    pub command: String, // 操作命令
}

impl ShadowsocksConfig {
    pub fn get_os(&self) -> String {
        self.os.clone()
    }
    pub fn get_config(&self) -> String {
        self.config.clone()
    }

    pub fn get_command(&self) -> String {
        self.command.clone()
    }
}
