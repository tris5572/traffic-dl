use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let content = get_data_from_url("https://httpbin.org/ip").await?;
    println!("{}", &content);

    Ok(())
}

/// 指定した url からデータを取得し、文字列を返す。
async fn get_data_from_url(url: &str) -> Result<String> {
    let response = reqwest::get(url).await?;
    let text = response.text().await?;

    Ok(text)
}
