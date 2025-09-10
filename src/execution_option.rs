// RunOption型の定義
use crate::datetime;
use crate::types::{Cli, Interval};

/// 実行時のオプションを保持する構造体
#[derive(Debug)]
pub struct ExecutionOption {
    pub datetime: datetime::DT,
    pub interval_h1: bool,
    pub interval_m5: bool,
    /// 常設トラカンを取得対象とするかどうか
    pub type_permanent: bool,
    /// CCTVトラカンを取得対象とするかどうか
    pub type_cctv: bool,
}

impl ExecutionOption {
    /// コマンドラインのオプションから、実行時のオプションを生成する
    pub fn from_cli(cli: &Cli) -> Self {
        // let interval = if cli.m5 { Interval::M5 } else { Interval::H1 };

        let dt =
            datetime::parse(&cli.date).expect("日時指定が解釈不能。YYYYMMDD 形式による指定が必要");

        // TODO: 後で実装する
        // 取得対象のセンサーの種類である常設トラカンとCCTVトラカンを設定する
        // 基本的には両方とも対象とするが、片方のみが実行時に指定された場合はそちらのみを対象にする。
        let type_permanent = true;
        let type_cctv = true;

        let execution_option = ExecutionOption {
            datetime: dt,
            interval_h1: cli.h1,
            interval_m5: cli.m5,
            type_permanent: type_permanent,
            type_cctv: type_cctv,
        };

        execution_option
    }
}
