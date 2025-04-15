use actix_web::http::header::ContentType;
use actix_web::web::Data;
use actix_web::{get, web, HttpResponse, Responder};
use mime;
use serde::Serialize;
use std::fs::read;
use std::path::PathBuf;

use crate::database::{Database, ForkStats};

pub struct AppState {
    index_path: PathBuf,
    chart_js_path: PathBuf,
    fork_database_path: String,
}

impl AppState {
    pub fn new(static_path: &PathBuf, fork_database_path: PathBuf) -> AppState {
        // build path to index.html
        let mut index_path = PathBuf::from(&static_path);
        index_path.push("index.html");

        // build path to char.js
        let mut chart_js_path = PathBuf::from(&static_path);
        chart_js_path.push("chart-v4.4.8.js");

        AppState {
            index_path,
            chart_js_path,
            fork_database_path: String::from(fork_database_path.to_str().unwrap()),
        }
    }
}

#[derive(Serialize)]
struct StatsResponse {
    mxcubecore: ForkStats,
    mxcubeweb: ForkStats,
}

fn file_respons(file_path: &PathBuf, content_type: ContentType) -> impl Responder {
    let data = read(file_path).unwrap();

    HttpResponse::Ok().content_type(content_type).body(data)
}

#[get("/")]
async fn index(data: Data<AppState>) -> impl Responder {
    file_respons(&data.index_path, ContentType::html())
}

#[get("/chart.js")]
async fn chart_js(data: Data<AppState>) -> impl Responder {
    file_respons(
        &data.chart_js_path,
        ContentType(mime::APPLICATION_JAVASCRIPT),
    )
}

#[get("/stats")]
async fn stats(data: Data<AppState>) -> impl Responder {
    let db = Database::connect(data.fork_database_path.as_str()).unwrap();
    let response = StatsResponse {
        mxcubecore: db.get_fork_stats("mxcubecore").unwrap(),
        mxcubeweb: db.get_fork_stats("mxcubeweb").unwrap(),
    };
    web::Json(response)
}
