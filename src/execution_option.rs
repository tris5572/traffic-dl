use anyhow::{Context, Result};

use crate::datetime;
use crate::types::Cli;

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
    /// コマンドラインの実行時オプションから、実際のコード実行時のオプションを生成する
    pub fn from_args(args: &Cli) -> Result<Self> {
        let dt = datetime::parse(&args.date)
            .with_context(|| format!("{} を日時指定として解釈不能", args.date))?;

        // TODO: 後で実装する
        // 取得対象のセンサーの種類である常設トラカンとCCTVトラカンを設定する
        // 基本的には両方とも対象とするが、片方のみが実行時に指定された場合はそちらのみを対象にする。
        let type_permanent = true;
        let type_cctv = true;

        let execution_option = ExecutionOption {
            datetime: dt,
            interval_h1: args.h1,
            interval_m5: args.m5,
            type_permanent: type_permanent,
            type_cctv: type_cctv,
        };

        Ok(execution_option)
    }
}
