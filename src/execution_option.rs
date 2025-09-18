use anyhow::{Context, Result};

use crate::datetime;
use crate::types::{Cli, RoadType};

/// 実行時のオプションを保持する構造体
#[derive(Debug)]
pub struct ExecutionOption {
    /// 日時指定
    pub datetime: datetime::DT,

    /// 取得間隔：1時間ごと
    pub interval_h1: bool,
    /// 取得間隔：5分ごと
    pub interval_m5: bool,

    /// 観測機器：常設トラカンを取得対象とするかどうか
    pub type_permanent: bool,
    /// 観測機器：CCTVトラカンを取得対象とするかどうか
    pub type_cctv: bool,

    /// 道路種別：高速自動車国道を取得対象とするかどうか
    pub road_highway: bool,
    /// 道路種別：一般国道を取得対象とするかどうか
    pub road_normal: bool,

    /// 先頭の1つのみを対象とするかどうか
    pub one: bool,
    /// 実際のデータ取得を行わないドライランを行うかどうか
    pub dry: bool,
}

impl ExecutionOption {
    /// コマンドラインの実行時オプションから、実際のコード実行時のオプションを生成する
    pub fn from_args(args: &Cli) -> Result<Self> {
        let dt = datetime::parse(&args.date).with_context(|| format!("{} を日時指定として解釈不能", args.date))?;

        // 取得間隔
        // - 未指定時は1時間ごとのデータのみを取得
        // - `--5m` 指定時は、5分間ごとのデータのみを取得
        // - `--1h` と `--5m` の両方指定時は、両方のデータを取得
        let h1 = if !args.h1 && args.m5 { false } else { true };
        let m5 = if args.m5 { true } else { false };

        // 取得対象のセンサー。常設トラカンとCCTVトラカン
        // 基本的には両方とも対象とするが、片方のみが実行時に指定された場合はそちらのみを対象にする。
        let type_permanent = if !args.permanent && args.cctv { false } else { true };
        let type_cctv = if args.permanent && !args.cctv { false } else { true };

        // 道路種別
        // 未指定時は両方を対象とするが、片方のみが実行時に指定された場合はそちらのみを対象にする。
        let road_highway = if !args.highway && args.normal { false } else { true };
        let road_normal = if args.highway && !args.normal { false } else { true };

        let execution_option = ExecutionOption {
            datetime: dt,
            interval_h1: h1,
            interval_m5: m5,
            type_permanent: type_permanent,
            type_cctv: type_cctv,
            road_highway: road_highway,
            road_normal: road_normal,
            one: args.one,
            dry: args.dry,
        };

        Ok(execution_option)
    }

    /// 道路種別の enum を取得する
    pub fn road_type(&self) -> RoadType {
        if self.road_highway && !self.road_normal {
            RoadType::Highway
        } else if !self.road_highway && self.road_normal {
            RoadType::Normal
        } else {
            // 両方 true のときと、実際には存在しないはずの両方とも false のとき
            RoadType::Both
        }
    }
}

#[cfg(test)]
mod execute_option_from_args_test {
    use super::*;

    /// テスト用の Cli 構造体を生成する
    fn default_args() -> Cli {
        Cli {
            date: "20250102".to_string(),
            h1: true,
            m5: false,
            permanent: false,
            cctv: false,
            highway: true,
            normal: true,
            one: false,
            dry: false,
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
            args.h1 = false;
            args.m5 = false;
            let result = ExecutionOption::from_args(&args).unwrap();

            assert!(result.interval_h1);
            assert!(!result.interval_m5);
        }

        #[test]
        fn only_1h() {
            let mut args = default_args();
            args.h1 = true;
            args.m5 = false;
            let result = ExecutionOption::from_args(&args).unwrap();

            assert!(result.interval_h1);
            assert!(!result.interval_m5);
        }

        #[test]
        fn only_5m() {
            let mut args = default_args();
            args.h1 = false;
            args.m5 = true;
            let result = ExecutionOption::from_args(&args).unwrap();

            assert!(!result.interval_h1);
            assert!(result.interval_m5);
        }

        #[test]
        fn both() {
            let mut args = default_args();
            args.h1 = true;
            args.m5 = true;
            let result = ExecutionOption::from_args(&args).unwrap();

            assert!(result.interval_h1);
            assert!(result.interval_m5);
        }
    }

    #[cfg(test)]
    mod 取得対象 {
        use super::*;

        #[test]
        fn nothing() {
            let mut args = default_args();
            args.permanent = false;
            args.cctv = false;
            let result = ExecutionOption::from_args(&args).unwrap();

            assert!(result.type_permanent);
            assert!(result.type_cctv);
        }

        #[test]
        fn permanent() {
            let mut args = default_args();
            args.permanent = true;
            args.cctv = false;
            let result = ExecutionOption::from_args(&args).unwrap();

            assert!(result.type_permanent);
            assert!(!result.type_cctv);
        }

        #[test]
        fn cctv() {
            let mut args = default_args();
            args.permanent = false;
            args.cctv = true;
            let result = ExecutionOption::from_args(&args).unwrap();

            assert!(!result.type_permanent);
            assert!(result.type_cctv);
        }

        #[test]
        fn both() {
            let mut args = default_args();
            args.permanent = true;
            args.cctv = true;
            let result = ExecutionOption::from_args(&args).unwrap();

            assert!(result.type_permanent);
            assert!(result.type_cctv);
        }
    }

    #[cfg(test)]
    mod 道路種別 {
        use super::*;

        #[test]
        fn nothing() {
            let mut args = default_args();
            args.highway = false;
            args.normal = false;
            let result = ExecutionOption::from_args(&args).unwrap();

            assert!(result.road_highway);
            assert!(result.road_normal);
        }

        #[test]
        fn highway() {
            let mut args = default_args();
            args.highway = true;
            args.normal = false;
            let result = ExecutionOption::from_args(&args).unwrap();

            assert!(result.road_highway);
            assert!(!result.road_normal);
        }

        #[test]
        fn cctv() {
            let mut args = default_args();
            args.highway = false;
            args.normal = true;
            let result = ExecutionOption::from_args(&args).unwrap();

            assert!(!result.road_highway);
            assert!(result.road_normal);
        }

        #[test]
        fn both() {
            let mut args = default_args();
            args.highway = true;
            args.normal = true;
            let result = ExecutionOption::from_args(&args).unwrap();

            assert!(result.road_highway);
            assert!(result.road_normal);
        }
    }
}
