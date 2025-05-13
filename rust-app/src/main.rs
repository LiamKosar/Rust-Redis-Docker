use std::env;

mod app;
mod celery;
pub mod config;
mod worker;
use crate::app::run_app;
use crate::config::{APP_RUN_MODE, WORKER_RUN_MODE};
use crate::worker::run_worker;
use celerylib::print_meow;

fn main() {
    // Check environment variable
    let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "unknown".to_string());

    print_meow();
    match run_mode.as_str() {
        APP_RUN_MODE => run_app(),
        WORKER_RUN_MODE => run_worker(),
        _ => {
            eprintln!("Unknown mode. Set RUN_MODE to 'app' or 'worker'");
            std::process::exit(1);
        }
    }
}
