use anyhow::Result;
use clap::Parser;

mod datetime;
mod run_option;
mod types;
mod url;

// const TEST_IP_URL: &str = "https://httpbin.org/ip";
// const TEST_DATA_URL: &str = "https://api.jartic-open-traffic.org/geoserver?service=WFS&version=2.0.0&request=GetFeature&typeNames=t_travospublic_measure_1h&srsName=EPSG:4326&outputFormat=application/json&exceptions=application/json&cql_filter=道路種別='3' AND 時間コード=202509010900 AND 常時観測点コード=3310840";

#[tokio::main]
async fn main() -> Result<()> {
    let args = types::Cli::parse();

    // RunOptionの作成
    let run_option = run_option::RunOption::from_cli(&args);

    let url = url::create_url(run_option.datetime, run_option.interval);
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
