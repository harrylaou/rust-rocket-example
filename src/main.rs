#![feature(proc_macro_hygiene, decl_macro)]

mod request_models;
mod web_client;

use crate::request_models::time_series::TimeSeries;
use crate::web_client::worldometers::*;
use rocket::{self, get, routes};
use rocket_contrib::json::Json;

#[get("/")]
async fn index() -> Json<TimeSeries> {
    let time_series = get_time_series().await.unwrap();
    Json(time_series)
}

fn main() {
    let _ = rocket::ignite().mount("/", routes![index]).launch();
}
