use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeries {
    pub data: Vec<Area>,
    pub dt: String,
    pub ts: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Area {
    #[serde(rename = "Province/State")]
    pub province_state: String,
    #[serde(rename = "Country/Region")]
    pub country_region: String,
    #[serde(rename = "Coordinates")]
    pub coordinates: Coordinates,
    #[serde(rename = "TimeSeries")]
    pub time_series: Vec<TimeSery>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coordinates {
    #[serde(rename = "Lat")]
    pub lat: f64,
    #[serde(rename = "Long")]
    pub long: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeSery {
    pub date: String,
    pub value: i64,
}
