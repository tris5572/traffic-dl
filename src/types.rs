use clap::Parser;

/// コマンド実行時のオプション定義
#[derive(Parser)]
pub struct Cli {
    /// YYYYMMDDフォーマットの日付
    pub date: String,

    /// 取得間隔：1時間ごとのデータを取得 (デフォルト)
    #[arg(long = "1h")]
    pub h1: bool,
    /// 取得間隔：5分ごとのデータを取得。これのみの指定時、1時間ごとのデータ(--1h)を取得しない
    #[arg(long = "5m")]
    pub m5: bool,

    /// 観測機器：常設トラカンのみを取得対象とする
    #[arg(long = "permanent")]
    pub permanent: bool,
    /// 観測機器：CCTVトラカンのみを取得対象とする
    #[arg(long = "cctv")]
    pub cctv: bool,

    /// 先頭の1つのデータのみ取得・保存を実行する
    #[arg(long = "one")]
    pub one: bool,
    /// ドライランとして実行し、データの取得・保存を行わない
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

/// 観測機器
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
