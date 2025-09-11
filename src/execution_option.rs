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
    /// 高速自動車国道を取得対象とするかどうか
    pub road_highway: bool,
    /// 一般国道を取得対象とするかどうか
    pub road_normal: bool,
}

impl ExecutionOption {
    /// コマンドラインの実行時オプションから、実際のコード実行時のオプションを生成する
    pub fn from_args(args: &Cli) -> Result<Self> {
        let dt = datetime::parse(&args.date)
            .with_context(|| format!("{} を日時指定として解釈不能", args.date))?;

        // 取得間隔
        // - 未指定時は1時間ごとのデータのみを取得
        // - `--5m` 指定時は、5分間ごとのデータのみを取得
        // - `--1h` と `--5m` の両方指定時は、両方のデータを取得
        let h1 = if args.h1.is_none() && args.m5.is_some() {
            false
        } else {
            true
        };
        let m5 = if args.m5.is_some() { true } else { false };

        // TODO: 後で実装する
        // 取得対象のセンサーの種類である常設トラカンとCCTVトラカンを設定する
        // 基本的には両方とも対象とするが、片方のみが実行時に指定された場合はそちらのみを対象にする。
        let type_permanent = true;
        let type_cctv = true;

        // TODO: 道路種別の判定を追加する
        let road_highway = false;
        let road_normal = true;

        let execution_option = ExecutionOption {
            datetime: dt,
            interval_h1: h1,
            interval_m5: m5,
            type_permanent: type_permanent,
            type_cctv: type_cctv,
            road_highway: road_highway,
            road_normal: road_normal,
        };

        Ok(execution_option)
    }
}

#[cfg(test)]
mod execute_option_from_args_test {
    use super::*;

    /// テスト用の Cli 構造体を生成する
    fn default_args() -> Cli {
        Cli {
            date: "20250102".to_string(),
            h1: Some(true),
            m5: None,
            permanent: None,
            cctv: None,
        }
    }

    #[cfg(test)]
    mod 日時指定 {
        use super::*;

        #[test]
        fn valid() {
            let mut args = default_args();

            args.date = "20250901".into();
            assert!(ExecutionOption::from_args(&args).is_ok());
        }

        #[test]
        fn invalid() {
            let mut args = default_args();

            args.date = "2025090".into();
            assert!(ExecutionOption::from_args(&args).is_err());

            args.date = "202509".into();
            assert!(ExecutionOption::from_args(&args).is_err());

            args.date = "20251".into();
            assert!(ExecutionOption::from_args(&args).is_err());

            args.date = "2025".into();
            assert!(ExecutionOption::from_args(&args).is_err());

            args.date = "abc".into();
            assert!(ExecutionOption::from_args(&args).is_err());

            args.date = "".into();
            assert!(ExecutionOption::from_args(&args).is_err());
        }
    }

    #[cfg(test)]
    mod 間隔指定 {
        use super::*;

        #[test]
        fn unspecified() {
            let mut args = default_args();
            args.h1 = None;
            args.m5 = None;
            let result = ExecutionOption::from_args(&args).unwrap();

            assert!(result.interval_h1);
            assert!(!result.interval_m5);
        }

        #[test]
        fn only_1h() {
            let mut args = default_args();
            args.h1 = Some(true);
            args.m5 = None;
            let result = ExecutionOption::from_args(&args).unwrap();

            assert!(result.interval_h1);
            assert!(!result.interval_m5);
        }

        #[test]
        fn only_5m() {
            let mut args = default_args();
            args.h1 = None;
            args.m5 = Some(true);
            let result = ExecutionOption::from_args(&args).unwrap();

            assert!(!result.interval_h1);
            assert!(result.interval_m5);
        }

        #[test]
        fn both() {
            let mut args = default_args();
            args.h1 = Some(true);
            args.m5 = Some(true);
            let result = ExecutionOption::from_args(&args).unwrap();

            assert!(result.interval_h1);
            assert!(result.interval_m5);
        }
    }
}
