pub mod worldometers {
    use crate::request_models::time_series::TimeSeries;

    pub async fn get_time_series() -> Result<TimeSeries, reqwest::Error> {
        let client = reqwest::Client::new();
        let response = client
            .get("https://covid2019-api.herokuapp.com/v2/timeseries/confirmed")
            .send()
            .await?;
        println!("Status: {}", response.status());

        let time_series_result: Result<TimeSeries, reqwest::Error> = response.json().await;
        let time_series_result_ref: Result<&TimeSeries, &reqwest::Error> =
            time_series_result.as_ref();
        println!(
            "timeseries {}",
            time_series_result_ref.map_or("no timeseries", |_| "works")
        );
        time_series_result
    }
}
