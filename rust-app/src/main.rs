use std::env;
pub mod constants;

mod app;
mod our_redis;
mod worker;

use crate::app::run_app;
use crate::constants::{APP_RUN_MODE, WORKER_RUN_MODE};
use crate::worker::run_worker;

fn main() {
    // Check environment variable
    let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "unknown".to_string());

    match run_mode.as_str() {
        APP_RUN_MODE => run_app(),
        WORKER_RUN_MODE => run_worker(),
        _ => {
            eprintln!("Unknown mode. Set RUN_MODE to 'app' or 'worker'");
            std::process::exit(1);
        }
    }
}
