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

    /// 先頭の1つのファイルだけ取得・保存を実行する
    #[arg(long = "one")]
    pub one: bool,
    /// ドライランとして、データの取得・保存を行わない
    #[arg(long = "dry")]
    pub dry: bool,
}

/// データの取得間隔
#[derive(Debug)]
pub enum Interval {
    /// 1時間ごと
    H1,
    /// 5分ごと
    M5,
}

/// データの取得対象
#[derive(Debug)]
pub enum CounterType {
    /// 常設トラカン
    Permanent,
    /// CCTVトラカン
    Cctv,
}

/// 道路種別
#[derive(Debug)]
pub enum RoadType {
    /// 高速自動車国道
    Highway,
    /// 一般国道
    Normal,
}
