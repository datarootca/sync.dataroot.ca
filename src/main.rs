use api::lib::Scheduler;
use dotenv::dotenv;
use std::{sync::Arc};

use infrastructure::repository::{postgres::{postgres}, sync};
mod api;
mod domain;
mod infrastructure;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let pg_pool_result = postgres::init();
    if pg_pool_result.is_err() {
        log::error!("{}", pg_pool_result.unwrap_err());
        std::process::exit(1)
    }
    let sync_pool_result = sync::postgres::init();
    if sync_pool_result.is_err() {
        log::error!("{}", pg_pool_result.unwrap_err());
        std::process::exit(1)
    }
    let pg_pool = Arc::new(pg_pool_result.unwrap());

    let sync_pool = Arc::new(sync_pool_result.unwrap());

    let scheduler = Scheduler::new(pg_pool.clone(),sync_pool.clone());

    let result = scheduler.run().await;
    if result.is_err() {
        log::error!("{}", result.unwrap_err().to_string());
        std::process::exit(1)
    }
}
