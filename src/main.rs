use api::lib;
use dotenv::dotenv;
use std::{io, sync::Arc};

use infrastructure::repository::postgres::{postgres};
mod api;
mod domain;
mod infrastructure;

#[tokio::main]
async fn main() {
    print!("beforenv");
    dotenv().ok();
    print!("test env");
    env_logger::init();
    print!("test");

    let pg_pool_result = postgres::init();
    if pg_pool_result.is_err() {
        log::error!("{}", pg_pool_result.unwrap_err());
        std::process::exit(1)
    }
    print!("test");
    let pg_pool = Arc::new(pg_pool_result.unwrap());

    let result = lib::run(pg_pool.clone()).await;
    if result.is_err() {
        log::error!("{}", result.unwrap_err().to_string());
        std::process::exit(1)
    }
}
