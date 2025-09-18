use crate::datetime::DT;
use crate::execution_option::ExecutionOption;
use crate::types::*;

const URL_1: &str = "https://api.jartic-open-traffic.org/geoserver?service=WFS&version=2.0.0&request=GetFeature&typeNames=";
const URL_2: &str = "&srsName=EPSG:4326&outputFormat=application/json&exceptions=application/json&cql_filter=";

/// ファイル名と取得先 URL のタプルのリストを生成する
pub fn create_names_and_urls(datetime: DT, option: &ExecutionOption) -> Vec<(String, String)> {
    let mut output = vec![];

    // 1時間ごとのデータ取得時
    if option.interval_h1 {
        match datetime {
            DT::YMD { .. } | DT::YMDH { .. } => {
                let list = get_datetime_list_1h(&datetime);
                let road_type = option.road_type();

                for t in list {
                    if option.type_permanent {
                        output.push(get_target(&t, &Interval::H1, &road_type, &CounterType::Permanent));
                    }
                    if option.type_cctv {
                        output.push(get_target(&t, &Interval::H1, &road_type, &CounterType::Cctv));
                    }
                }
            }
            _ => {}
        }
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
        _ => vec![],
    }
}

/// 保存に使用するファイル名と取得先URLを取得する
fn get_target(time: &str, interval: &Interval, road_type: &RoadType, counter_type: &CounterType) -> (String, String) {
    let name = create_filename(time, &interval, &road_type, &counter_type);
    let url = create_url(time, &interval, &road_type, &counter_type);

    (name, url)
}

/// 保存に使用するファイル名(拡張子なし)を生成する
fn create_filename(time: &str, interval: &Interval, _road_type: &RoadType, counter_type: &CounterType) -> String {
    let itv = match interval {
        Interval::H1 => "H",
        Interval::M5 => "M",
    };

    let cnt = match counter_type {
        CounterType::Permanent => "P",
        CounterType::Cctv => "C",
    };

    // NOTE: 道路種別はまとめて取得するため、ファイル名には反映しない

    format!("{}{}{}", itv, time, cnt)
}

/// 取得対象のURLを生成する
fn create_url(time: &str, interval: &Interval, road_type: &RoadType, counter_type: &CounterType) -> String {
    // 取得対象データの種別。カウンターの種類と間隔に基づく
    let target = match counter_type {
        CounterType::Permanent => match interval {
            Interval::H1 => "t_travospublic_measure_1h",
            Interval::M5 => "t_travospublic_measure_5m",
        },
        CounterType::Cctv => match interval {
            Interval::H1 => "t_travospublic_measure_1h_img",
            Interval::M5 => "t_travospublic_measure_5m_img",
        },
    };

    let road = match road_type {
        RoadType::Highway => "道路種別='1'",
        RoadType::Normal => "道路種別='3'",
        RoadType::Both => "道路種別='1' OR 道路種別='3'",
    };

    // NOTE: デバッグ用に常時観測点コードを絞る場合は、以下のように設定する
    // "{}{}{}道路種別='{}' AND 時間コード={} AND 常時観測点コード=3310840",

    // NOTE: 道路種別は OR 条件で複数指定されることがあるため、 () でくくる
    format!("{}{}{}({}) AND 時間コード={}", URL_1, target, URL_2, road, time)
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
