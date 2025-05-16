use std::env;

mod app;
mod celery;
pub mod config;
mod worker;
use crate::app::run_app;
use crate::config::{APP_RUN_MODE, WORKER_RUN_MODE};
use crate::worker::run_worker;
use celerylib::{Celery, Task};

fn main() {
    // Check environment variable
    let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "unknown".to_string());

    let mut celery: Celery = Celery::create_celery("redis://redis/").unwrap();
    let task: Box<dyn Task<i32>> = Box::new(CoolTask{});
    let depth = celery.push_task(task, 42).unwrap();

    println!("Depth: {}", depth);
    // match run_mode.as_str() {
    //     APP_RUN_MODE => run_app(),
    //     WORKER_RUN_MODE => run_worker(),
    //     _ => {
    //         eprintln!("Unknown mode. Set RUN_MODE to 'app' or 'worker'");
    //         std::process::exit(1);
    //     }
    // }
}

struct CoolTask  {}

impl Task<i32> for CoolTask {
    fn get_task_name(&self) -> &str {
        return "cool_task";
    }
}