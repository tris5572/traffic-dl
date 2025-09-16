use anyhow::Result;
use clap::Parser;
use tokio::time::{Duration, sleep};

mod datetime;
mod execution_option;
mod types;
mod url;

// const TEST_IP_URL: &str = "https://httpbin.org/ip";
// const TEST_DATA_URL: &str = "https://api.jartic-open-traffic.org/geoserver?service=WFS&version=2.0.0&request=GetFeature&typeNames=t_travospublic_measure_1h&srsName=EPSG:4326&outputFormat=application/json&exceptions=application/json&cql_filter=道路種別='3' AND 時間コード=202509010900 AND 常時観測点コード=3310840";

#[tokio::main]
async fn main() -> Result<()> {
    let args = types::Cli::parse();

    let execute_option = execution_option::ExecutionOption::from_args(&args)?;

    let dt = datetime::parse(&args.date).expect("日時指定が不正");

    let names_and_urls = url::create_names_and_urls(dt, &execute_option);

    for (name, url) in names_and_urls {
        if execute_option.dry {
            println!("{} - {}", &name, &url);
        } else {
            // 実際にデータを取得してファイルとして保存する
            let content = get_data_from_url(&url).await?;
            let path = format!("{}.json", name);
            save_to_file(&path, "data", &content).await?;

            if execute_option.one {
                // `--one` が指定されているときは、最初の1つのみを処理して終了する
                break;
            } else {
                // 取得頻度を下げるために間隔を開ける
                sleep(Duration::from_secs(1)).await;
            }
        }
    }

    Ok(())
}

/// 指定した url からデータを取得し、文字列を返す。
async fn get_data_from_url(url: &str) -> Result<String> {
    let response = reqwest::get(url).await?;
    let text = response.text().await?;

    Ok(text)
}

/// データを指定フォルダへ保存する。
async fn save_to_file(filename: &str, dir: &str, content: &str) -> Result<()> {
    // 出力先ディレクトリが存在しないときは作成する
    if !std::path::Path::exists(&std::path::Path::new(dir)) {
        std::fs::create_dir(dir)?;
    }

    let path = format!("{}/{}", dir, filename);
    tokio::fs::write(&path, content).await?;
    Ok(())
}
