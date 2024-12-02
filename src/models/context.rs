use std::path::Path;

pub trait ShadowsocksPort {
    fn read_shadowsocks_port(&self, file_path: &Path) -> Result<u32, Box<dyn std::error::Error>>;

    fn modify_shadowsocks_port(
        &self,
        file_path: &Path,
        port: u32,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct Context {
    shadowsocks_port: Box<dyn ShadowsocksPort>,
}

impl Context {
    pub fn new(shadowsocks_port: Box<dyn ShadowsocksPort>) -> Self {
        Context { shadowsocks_port }
    }

    pub fn read_shadowsocks_port(
        &self,
        file_path: &Path,
    ) -> Result<u32, Box<dyn std::error::Error>> {
        self.shadowsocks_port.read_shadowsocks_port(file_path)
    }

    pub fn modify_shadowsocks_port(
        &self,
        file_path: &Path,
        port: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.shadowsocks_port
            .modify_shadowsocks_port(file_path, port)
    }
}
