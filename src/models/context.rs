use async_trait::async_trait;
use std::path::Path;

#[async_trait]
pub trait ShadowsocksPort {
    async fn read_shadowsocks_port(
        &self,
        file_path: &Path,
    ) -> Result<u32, Box<dyn std::error::Error>>;

    async fn modify_shadowsocks_port(
        &self,
        file_path: &Path,
        port: u32,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct Context {
    pub shadowsocks_port: Box<dyn ShadowsocksPort + Send>,
}

impl Context {
    pub fn new(shadowsocks_port: Box<dyn ShadowsocksPort + Send>) -> Self {
        Context { shadowsocks_port }
    }
}
