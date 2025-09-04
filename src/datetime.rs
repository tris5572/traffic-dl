use chrono::{Datelike, NaiveDate, NaiveDateTime};

#[derive(Debug, PartialEq)]
pub enum DT {
    YMD {
        string: String,
        year: i32,
        month: u32,
        day: u32,
    },
}

pub fn parse(input: &str) -> Option<DT> {
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
