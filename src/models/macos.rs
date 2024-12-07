use crate::context::ShadowsocksPort;
use async_trait::async_trait;
use serde_json::Value;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use tracing::debug;

pub struct MacOS;

#[async_trait]
impl ShadowsocksPort for MacOS {
    async fn read_shadowsocks_port(&self, file_path: &Path) -> Result<u32, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut parsed_json: Value = serde_json::from_reader(reader)?;
        debug!("read local shadowsocks config file : {:#?}", parsed_json);

        if !parsed_json.is_null() {
            if let Some(server_port) = parsed_json.get_mut("server_port") {
                return Ok(server_port.as_u64().unwrap_or(0) as u32);
            }
        }
        Ok(0)
    }

    async fn modify_shadowsocks_port(
        &self,
        file_path: &Path,
        port: u32,
    ) -> Result<(), Box<dyn Error>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut parsed_json: Value = serde_json::from_reader(reader)?;

        if let Some(server_port) = parsed_json.get_mut("server_port") {
            *server_port = Value::Number(serde_json::Number::from(port));
            debug!("read local shadowsocks config file : {:#?}", parsed_json);
        }

        let modified_json_string = serde_json::to_string_pretty(&parsed_json)?;
        fs::write(file_path, modified_json_string)?;

        Ok(())
    }
}
