use clap::Parser;

/// コマンド実行時のオプション定義
#[derive(Parser)]
pub struct Cli {
    /// YYYYMMDDフォーマットの日付
    pub date: String,

    /// 1時間ごとのデータを取得 (デフォルト)
    #[arg(long = "1h")]
    pub h1: Option<bool>,
    /// 5分間隔のデータを取得。これのみの指定時、1時間ごとのデータ(--1h)を取得しない
    #[arg(long = "5m")]
    pub m5: Option<bool>,

    /// 常設トラカンのみを取得対象とする
    #[arg(long = "permanent")]
    pub permanent: Option<bool>,
    /// CCTVトラカンのみを取得対象とする
    #[arg(long = "cctv")]
    pub cctv: Option<bool>,
}
