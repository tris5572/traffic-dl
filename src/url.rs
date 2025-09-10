use crate::datetime::DT;
use crate::types::*;

const URL_1H: &str = "https://api.jartic-open-traffic.org/geoserver?service=WFS&version=2.0.0&request=GetFeature&typeNames=t_travospublic_measure_1h&srsName=EPSG:4326&outputFormat=application/json&exceptions=application/json&cql_filter=";
const URL_5M: &str = "https://api.jartic-open-traffic.org/geoserver?service=WFS&version=2.0.0&request=GetFeature&typeNames=t_travospublic_measure_5m&srsName=EPSG:4326&outputFormat=application/json&exceptions=application/json&cql_filter=";

/// 取得対象の URL を返す仮実装
pub fn create_url(input: DT, interval: Interval) -> String {
    let base_url = match interval {
        Interval::M5 => URL_5M,
        Interval::H1 => URL_1H,
    };

    let datetime_list = get_datetime_list_1h(&input);

    if 0 < datetime_list.len() {
        format!(
            "{}道路種別='3' AND 時間コード={} AND 常時観測点コード=3310840",
            base_url, datetime_list[0]
        )
    } else {
        "".into()
    }
}

/// ファイル名と取得先 URL のタプルのリストを生成する
pub fn create_names_and_urls(datetime: DT, cli: &Cli) -> Vec<(String, String)> {
    let mut output = vec![];

    match datetime {
        DT::YMD { ref string, .. } => {
            let list = get_datetime_list_1h(&datetime);
            for t in list {
                let name = format!("{}", &string);
                let url = format!(
                    "{}道路種別='3' AND 時間コード={} AND 常時観測点コード=3310840",
                    URL_1H, t
                );
                output.push((name, url));
            }
        }
        DT::YMDH { string, .. } => {}
    }

    output
}

/// 1時間ごとのデータを取得するため、取得対象日時の配列を生成する
/// - 年月日のみが指定されている場合は、1日分のリストを返す
/// - 年月日と時が指定されている場合は、1時間分のみを返す
/// - それ以外の場合は空配列を返す
pub fn get_datetime_list_1h(dt: &DT) -> Vec<String> {
    match dt {
        DT::YMD { string, .. } => {
            let mut output = Vec::new();
            for hour in 0..24 {
                let hour_str = format!("{:02}", hour);
                output.push(format!("{}{}00", string, hour_str));
            }
            output
        }
        DT::YMDH { string, .. } => {
            vec![format!("{}00", string)]
        }
    }
}

#[cfg(test)]
mod get_datetime_list_1h_tests {
    use super::*;

    #[test]
    fn test_get_datetime_list_1h_for_ymd() {
        let dt = DT::YMD {
            string: "20250102".to_string(),
            year: 2025,
            month: 1,
            day: 2,
        };

        let result = get_datetime_list_1h(&dt);

        // 24時間分のデータが返されることを確認
        assert_eq!(result.len(), 24);

        // 各要素が正しい形式であること確認
        for (i, datetime) in result.iter().enumerate() {
            let expected_hour = format!("{:02}", i);
            assert_eq!(datetime, &format!("20250102{}00", expected_hour));
        }
    }

    #[test]
    fn test_get_datetime_list_1h_for_ymdh() {
        let dt = DT::YMDH {
            string: "2025010203".to_string(),
            year: 2025,
            month: 1,
            day: 2,
            hour: 3,
        };

        let result = get_datetime_list_1h(&dt);

        // 1時間分のみが返されることを確認
        assert_eq!(result.len(), 1);

        // 返された要素が正しい形式であること確認
        assert_eq!(result[0], "202501020300");
    }
}
