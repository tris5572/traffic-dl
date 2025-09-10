use clap::Parser;

/// コマンド実行時のオプション定義
#[derive(Parser)]
pub struct Cli {
    /// YYYYMMDDフォーマットの日付
    pub date: String,
    /// 1時間ごとのデータを取得 (デフォルト)
    #[arg(long = "1h", default_value_t = true)]
    pub h1: bool,
    /// 5分間隔のデータを取得。この指定時、1時間ごとのデータ(--1h)を取得しない
    #[arg(long = "5m", conflicts_with = "h1")]
    pub m5: bool,

    /// 常設トラカンのみを取得対象とする
    #[arg(long = "permanent")]
    pub permanent: Option<bool>,
    /// CCTVトラカンのみを取得対象とする
    #[arg(long = "cctv")]
    pub cctv: Option<bool>,
}

/// データの取得間隔
#[derive(Debug)]
pub enum Interval {
    /// 5分ごと
    M5,
    /// 1時間ごと
    H1,
}
