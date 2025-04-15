use actix_web::web::Data;
use actix_web::{App, HttpServer};
use clap::{arg, Command};
use std::fs::canonicalize;
use std::path::PathBuf;

use fork::routes::{chart_js, index, stats, AppState};

///
/// returns path to static files and database file, as specified with CLI argument
///
fn args() -> (PathBuf, PathBuf) {
    let matches = Command::new("Fork Stats")
        .about("how deep does your fork go")
        .arg(arg!([static_path] "path to static files").required(true))
        .arg(arg!([fork_db] "path to fork.db").required(true))
        .get_matches();

    let static_path = canonicalize(matches.get_one::<String>("static_path").unwrap()).unwrap();
    let fork_db = canonicalize(matches.get_one::<String>("fork_db").unwrap()).unwrap();

    (static_path, fork_db)
}

fn build_app_state() -> AppState {
    let (static_path, fork_db) = args();

    AppState::new(&static_path, fork_db)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(build_app_state()))
            .service(index)
            .service(chart_js)
            .service(stats)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
