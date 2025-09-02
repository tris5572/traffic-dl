use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// YYYYMMDD フォーマットの日付
    date: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    let content = get_data_from_url("https://httpbin.org/ip").await?;

    // let filename = format!("{}.txt", args.date);
    let filename = "output.txt";
    save_to_file(&filename, &content).await?;

    Ok(())
}

/// 指定した url からデータを取得し、文字列を返す。
async fn get_data_from_url(url: &str) -> Result<String> {
    let response = reqwest::get(url).await?;
    let text = response.text().await?;

    Ok(text)
}

/// 指定した文字列をカレントディレクトリにファイルとして保存する。
async fn save_to_file(filename: &str, content: &str) -> Result<()> {
    tokio::fs::write(filename, content).await?;
    Ok(())
}
