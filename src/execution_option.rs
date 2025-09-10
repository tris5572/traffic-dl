// RunOption型の定義
use crate::datetime;
use crate::types::{Cli, Interval};

/// 実行時のオプションを保持する構造体
#[derive(Debug)]
pub struct ExecutionOption {
    pub datetime: datetime::DT,
    pub interval_h1: bool,
    pub interval_m5: bool,
}

impl ExecutionOption {
    /// コマンドラインのオプションから、実行時のオプションを生成する
    pub fn from_cli(cli: &Cli) -> Self {
        // let interval = if cli.m5 { Interval::M5 } else { Interval::H1 };

        let dt =
            datetime::parse(&cli.date).expect("日時指定が解釈不能。YYYYMMDD 形式による指定が必要");

        // ExecutionOption::new(dt, interval)
        ExecutionOption {
            datetime: dt,
            interval_h1: cli.h1,
            interval_m5: cli.m5,
        }
    }
}
