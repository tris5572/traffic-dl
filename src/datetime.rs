use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike};

#[derive(Debug, PartialEq)]
pub enum DT {
    YMD {
        string: String,
        year: i32,
        month: u32,
        day: u32,
    },
    YMDH {
        string: String,
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
    },
}

pub fn parse(input: &str) -> Option<DT> {
    // YYYYMMDDHH
    // chrono のパースでは「分」の指定が必須であるため、10文字だったときは`00` を追加して分があるものとして解釈してみる
    if input.len() == 10 {
        let result = NaiveDateTime::parse_from_str(&format!("{}00", input), "%Y%m%d%H%M");
        if let Ok(dt) = result {
            let dt = DT::YMDH {
                string: input.to_string(),
                year: dt.year(),
                month: dt.month(),
                day: dt.day(),
                hour: dt.hour(),
            };
            return Some(dt);
        }
    }

    // YYYYMMDD
    let result = NaiveDate::parse_from_str(input, "%Y%m%d");
    if let Ok(ymd) = result {
        let dt = DT::YMD {
            string: input.to_string(),
            year: ymd.year(),
            month: ymd.month(),
            day: ymd.day(),
        };
        return Some(dt);
    }

    // println!("Error: {:?}", result);
    None
}

#[cfg(test)]
mod parse_tests {
    use super::*;

    #[test]
    fn yyyymmddhh() {
        assert_eq!(
            parse("2025010203"),
            Some(DT::YMDH {
                string: "2025010203".into(),
                year: 2025,
                month: 1,
                day: 2,
                hour: 3,
            })
        );
        assert_eq!(
            parse("2025123123"),
            Some(DT::YMDH {
                string: "2025123123".into(),
                year: 2025,
                month: 12,
                day: 31,
                hour: 23,
            })
        );
        assert!(parse("2025010224").is_none());
    }

    #[test]
    fn yyyymmdd() {
        assert_eq!(
            parse("20250102"),
            Some(DT::YMD {
                string: "20250102".into(),
                year: 2025,
                month: 1,
                day: 2,
            })
        );
        assert_eq!(
            parse("20251231"),
            Some(DT::YMD {
                string: "20251231".into(),
                year: 2025,
                month: 12,
                day: 31,
            })
        );
        assert!(parse("20251301").is_none());
    }

    #[test]
    fn invalid() {
        assert!(parse("202501023").is_none());
        assert!(parse("202512").is_none());
        assert!(parse("20251").is_none());
        assert!(parse("2025").is_none());
        assert!(parse("abc").is_none());
        assert!(parse("").is_none());
    }
}
