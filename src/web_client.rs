pub mod worldometers {
    use crate::request_models::time_series::TimeSeries;

    pub async fn get_time_series() -> Result<TimeSeries, reqwest::Error> {
        let client = reqwest::Client::new();
        let response = client
            .get("https://covid2019-api.herokuapp.com/v2/timeseries/confirmed")
            .send()
            .await?;
        println!("Status: {}", response.status());

        let time_series: Result<TimeSeries, reqwest::Error> = response.json().await;
        time_series
    }
}
