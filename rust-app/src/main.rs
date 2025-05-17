use std::env;

mod app;
mod celery;
pub mod config;
mod worker;
use crate::app::run_app;
use crate::config::{APP_RUN_MODE, WORKER_RUN_MODE};
use crate::worker::run_worker;
use celerylib::{Celery, Task, TaskResult, TaskSuccess};

fn main() {
    // Check environment variable
    let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "unknown".to_string());

    let mut celery: Celery = Celery::create_celery("redis://redis/").unwrap();
    celery.register_task::<i32, CoolTask>();
    let depth = celery.push_task::<i32, CoolTask>(42).unwrap();
    let depth = celery.push_task::<i32, CoolTask>(42).unwrap();
    let depth = celery.push_task::<i32, CoolTask>(42).unwrap();
    let depth = celery.push_task::<i32, CoolTask>(42).unwrap();
    let depth = celery.push_task::<i32, CoolTask>(42).unwrap();
    let depth = celery.push_task::<i32, CoolTask>(42).unwrap();
    let depth = celery.push_task::<i32, CoolTask>(42).unwrap();
    let depth = celery.push_task::<i32, CoolTask>(42).unwrap();
    let depth = celery.push_task::<i32, CoolTask>(42).unwrap();
    let depth = celery.push_task::<i32, CoolTask>(42).unwrap();
    let depth = celery.push_task::<i32, CoolTask>(42).unwrap();
    let depth = celery.push_task::<i32, CoolTask>(42).unwrap();
    let depth = celery.push_task::<i32, CoolTask>(42).unwrap();
    let depth = celery.push_task::<i32, CoolTask>(42).unwrap();
    let depth = celery.push_task::<i32, CoolTask>(42).unwrap();
    let depth = celery.push_task::<i32, CoolTask>(42).unwrap();
    let depth = celery.push_task::<i32, CoolTask>(42).unwrap();
    let depth = celery.push_task::<i32, CoolTask>(42).unwrap();
    let depth = celery.push_task::<i32, CoolTask>(42).unwrap();
    let depth = celery.push_task::<i32, CoolTask>(42).unwrap();
    let depth = celery.push_task::<i32, CoolTask>(42).unwrap();
    let depth = celery.push_task::<i32, CoolTask>(42).unwrap();
    let depth = celery.push_task::<i32, CoolTask>(42).unwrap();
    let depth = celery.push_task::<i32, CoolTask>(42).unwrap();
    let depth = celery.push_task::<i32, CoolTask>(42).unwrap();
    let depth = celery.push_task::<i32, CoolTask>(42).unwrap();
    let depth = celery.push_task::<i32, CoolTask>(42).unwrap();
    let depth = celery.push_task::<i32, CoolTask>(42).unwrap();
    let depth = celery.push_task::<i32, CoolTask>(42).unwrap();

    
    println!("Depth: {}", depth);
    match run_mode.as_str() {
        APP_RUN_MODE => println!("My name is app :3"),
        WORKER_RUN_MODE => celery.run_worker(),
        _ => {
            eprintln!("Unknown mode. Set RUN_MODE to 'app' or 'worker'");
            std::process::exit(1);
        }
    }
}

struct CoolTask {}

impl Task<i32> for CoolTask {
    fn get_task_name() -> String {
        return "cool_task".to_string();
    }

    fn run(targs: i32) -> TaskResult {
        let res: i32 = Self::fib(targs);
        println!("Fib value: {}", res);
        return Ok(TaskSuccess{});
    }
    
}

impl CoolTask {
    fn fib(number: i32) -> i32 {
        if number == 0 {
            return 0;
        } else if number == 1 {
            return 1;
        }
        Self::fib(number - 1) + Self::fib(number - 2)
    }
}
