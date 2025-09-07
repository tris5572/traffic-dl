use anyhow::{Context, Result};
use clap::Parser;

mod datetime;
mod types;
mod url;

// const TEST_IP_URL: &str = "https://httpbin.org/ip";
// const TEST_DATA_URL: &str = "https://api.jartic-open-traffic.org/geoserver?service=WFS&version=2.0.0&request=GetFeature&typeNames=t_travospublic_measure_1h&srsName=EPSG:4326&outputFormat=application/json&exceptions=application/json&cql_filter=道路種別='3' AND 時間コード=202509010900 AND 常時観測点コード=3310840";

#[derive(Parser)]
struct Cli {
    /// YYYYMMDDフォーマットの日付
    date: String,
    /// 1時間ごとのデータを取得 (デフォルト)
    #[arg(long = "1h", default_value_t = true)]
    h1: bool,
    /// 5分間隔のデータを取得。この指定時、1時間ごとのデータ(--1h)を取得しない
    #[arg(long = "5m", conflicts_with = "h1")]
    m5: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    let interval = if args.m5 {
        types::Interval::M5
    } else {
        types::Interval::H1
    };

    // let content = get_data_from_url(TEST_DATA_URL).await?;
    let dt = datetime::parse(&args.date)
        .with_context(|| format!("日時指定が解釈不能。YYYYMMDD 形式による指定が必要"))?;

    let url = url::create_url(dt, interval);
    let content = get_data_from_url(&url).await?;

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
