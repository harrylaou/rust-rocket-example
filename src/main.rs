#![feature(proc_macro_hygiene, decl_macro)]

mod request_models;
mod web_client;

use crate::request_models::time_series::TimeSeries;
use crate::web_client::worldometers::*;
use rocket::{self, get, routes};
use rocket_contrib::json::Json;
// use job_scheduler::{Job, JobScheduler};
use job_scheduler::{Job, JobScheduler};
use rocket::{Rocket, State};
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
struct AppState {
    time_series: Arc<RwLock<TimeSeries>>,
}

#[get("/")]
async fn index(app_state: State<'_, Arc<AppState>>) -> Json<TimeSeries> {
    // let time_series = get_time_series().await.unwrap();
    let time_series_guard: RwLockReadGuard<TimeSeries> = app_state.inner().time_series.read().await;
    let time_series: TimeSeries = time_series_guard.to_owned();
    Json(time_series)
}

fn main() {
    let mut tokio_rt: Runtime = Runtime::new().unwrap();
    let time_series = tokio_rt.block_on(get_time_series()).unwrap();

    let app_state: Arc<AppState> = Arc::new(AppState {
        time_series: Arc::new(RwLock::new(time_series)),
    });

    // let app_state_job= app_state.clone();

    let rocket_app = rocket::ignite()
        .mount("/", routes![index])
        .manage(app_state.clone());

    // let app_state_external: State<AppState> = State::from(rocket_app.clone().as_ref()).unwrap();

    let mut job_scheduler = JobScheduler::new();
    job_scheduler.add(Job::new("0 * * * * *".parse().unwrap(), || {
        println!("job_scheduler executing");

        let time_series_result: Result<TimeSeries, reqwest::Error> =
            tokio_rt.block_on(get_time_series());
        match time_series_result {
            Ok(time_series) => {
                let time_series_arc = app_state.clone().time_series.clone();
                let mut time_series_guard: RwLockWriteGuard<TimeSeries> =
                    tokio_rt.block_on(time_series_arc.write());
                *time_series_guard = time_series;
            }
            Err(e) => eprintln!("Problem getting new timeseries: {}", e),
        }
    }));

    rocket_app.launch();
}
