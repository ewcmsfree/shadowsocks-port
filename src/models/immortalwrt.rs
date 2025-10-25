use crate::context::ShadowsocksPort;
use async_trait::async_trait;
use std::error::Error;
use std::fs::{File, read_to_string};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use tracing::debug;

pub struct Immortalwrt;

#[async_trait]
impl ShadowsocksPort for Immortalwrt {
    async fn read_shadowsocks_port(&self, file_path: &Path) -> Result<u32, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            if line.contains("option port") {
                let port = line.split_whitespace().nth(2).unwrap().replace("'", "");
                return Ok(port.parse::<u32>()?);
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

        let temp_file_path = file_path.with_extension("temp");
        let temp_file = File::create(&temp_file_path)?;
        let mut writer = BufWriter::new(temp_file);

        for line in reader.lines() {
            let line = line?;
            if line.contains("option port") {
                let mut new_line = line;
                new_line.replace_range(0..new_line.len(), &format!("	option port '{}'", port));
                writer.write_all(new_line.as_bytes())?;
            } else {
                writer.write_all(line.as_bytes())?;
            }
            writer.write_all(b"\n")?;
        }

        writer.flush()?;

        std::fs::remove_file(file_path)?;
        std::fs::rename(temp_file_path, file_path)?;

        debug!(
            "modify local shadowsocks config file : {:#?}",
            read_to_string(file_path)?
        );

        Ok(())
    }
}
