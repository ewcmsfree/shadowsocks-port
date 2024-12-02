use reqwest::Error;

/// 异步版本的文件读取函数
pub async fn read_file_async(url: &str) -> Result<String, Error> {
    let client = reqwest::Client::new();
    let res = client.get(url).send().await?;
    let body = res.text().await?;
    Ok(body)
}
