// RunOption型の定義
use crate::datetime;
use crate::types::{Cli, Interval};

#[derive(Debug)]
pub struct ExecutionOption {
    pub date: String,
    pub interval: Interval,
    pub datetime: datetime::DT,
}

impl ExecutionOption {
    /// ExecutionOptionを生成する
    pub fn new(date: String, interval: Interval, datetime: datetime::DT) -> Self {
        ExecutionOption {
            date,
            interval,
            datetime,
        }
    }

    /// Cli構造体からExecutionOptionを生成する
    pub fn from_cli(cli: &Cli) -> Self {
        let interval = if cli.m5 { Interval::M5 } else { Interval::H1 };

        let dt =
            datetime::parse(&cli.date).expect("日時指定が解釈不能。YYYYMMDD 形式による指定が必要");

        ExecutionOption::new(cli.date.clone(), interval, dt)
    }
}
