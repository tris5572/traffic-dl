use crate::datetime::DT;
use crate::types::*;

const URL_1H: &str = "https://api.jartic-open-traffic.org/geoserver?service=WFS&version=2.0.0&request=GetFeature&typeNames=t_travospublic_measure_1h&srsName=EPSG:4326&outputFormat=application/json&exceptions=application/json&cql_filter=";
const URL_5M: &str = "https://api.jartic-open-traffic.org/geoserver?service=WFS&version=2.0.0&request=GetFeature&typeNames=t_travospublic_measure_5m&srsName=EPSG:4326&outputFormat=application/json&exceptions=application/json&cql_filter=";

/// 取得対象の URL を返す仮実装
pub fn create_url(input: DT, interval: Interval) -> String {
    let datetime_str = match input {
        DT::YMD { string, .. } => format!("{}0000", string),
        DT::YMDH { string, .. } => format!("{}00", string),
    };

    let base_url = match interval {
        Interval::M5 => URL_5M,
        Interval::H1 => URL_1H,
    };

    format!(
        "{}道路種別='3' AND 時間コード={} AND 常時観測点コード=3310840",
        base_url, datetime_str
    )
}
