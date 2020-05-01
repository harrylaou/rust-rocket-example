#![feature(proc_macro_hygiene, decl_macro)]

mod request_models;
mod web_client;

use crate::request_models::time_series::TimeSeries;
use crate::web_client::worldometers::*;
use rocket::{self, get, routes};
use rocket_contrib::json::Json;
// use job_scheduler::{Job, JobScheduler};
use rocket::State;
use std::sync::{RwLock, RwLockReadGuard};
use tokio::runtime::Runtime;
struct AppState {
    time_series: RwLock<TimeSeries>,
}

#[get("/")]
async fn index(app_state: State<'_, AppState>) -> Json<TimeSeries> {
    // let time_series = get_time_series().await.unwrap();
    let time_series_guard: RwLockReadGuard<TimeSeries> =
        app_state.inner().time_series.read().unwrap();
    let time_series: TimeSeries = time_series_guard.to_owned();
    Json(time_series)
}

fn main() {
    // let mut job_scheduler = JobScheduler::new();
    let time_series = Runtime::new().unwrap().block_on(get_time_series()).unwrap();

    let app_state = AppState {
        time_series: RwLock::new(time_series),
    };

    // job_scheduler.add(Job::new("0 * * * * *".parse().unwrap(), || {
    //     println!("I get executed every minute");
    //     async {
    //         let time_series_result = get_time_series().await;
    //         match time_series_result {
    //             Ok(time_series) => {
    //                 let time_series_state_in = time_series_state.clone();
    //                 let mut time_series_guard = time_series_state_in.write().unwrap();
    //                 *time_series_guard = time_series;
    //             }
    //             Err(e) => eprintln!("Problem getting new timeseries: {}", e),
    //         }
    //     };
    // }));

    let _ = rocket::ignite()
        .mount("/", routes![index])
        .manage(app_state)
        .launch();
}
