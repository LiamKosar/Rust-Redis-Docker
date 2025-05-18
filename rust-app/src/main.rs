use std::env;

pub mod config;
use crate::config::{APP_RUN_MODE, WORKER_RUN_MODE};
use uuid::Uuid;
use celerylib::{Celery, Task, TaskError, TaskResult, TaskSuccess};
use serde::{Serialize, Deserialize};

use once_cell::sync::Lazy;
use std::sync::Mutex;


pub static GLOBAL_CELERY: Lazy<Mutex<Celery>> = Lazy::new(|| {
    Mutex::new(Celery::create_celery("redis://redis/").unwrap())
});

fn main() {
    // Check environment variable
    let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "unknown".to_string());

    let mut celery = GLOBAL_CELERY.lock().unwrap();
    celery.register_task::<i32, CoolTask>();
    celery.register_task::<Vec<i32>, HellaFib>();

    match run_mode.as_str() {
        APP_RUN_MODE => {
            for _ in 0..100 {
                celery.push_task::<Vec<i32>, HellaFib>(vec![42, 42, 42, 42, 42]).unwrap();
            }
        },
        WORKER_RUN_MODE => celery.run_worker(),
        _ => {
            eprintln!("Unknown mode. Set RUN_MODE to 'app' or 'worker'");
            std::process::exit(1);
        }
    }
}


// #[derive(Serialize, Deserialize)]

struct HellaFib {}

impl Task<Vec<i32>> for HellaFib {
    fn get_task_name() -> String {
        return "hella_fib".to_string();
    }

    fn run(numbers: Vec<i32>) -> TaskResult {

        for number in numbers {
            let res: i32 = CoolTask::fib(number);
            println!("Fib value: {}", res); 
        }
        
        return Ok(TaskSuccess{})
    }

}




struct CoolTask {}

impl Task<i32> for CoolTask {
    fn get_task_name() -> String {
        return "cool_task".to_string();
    }

    fn run(number: i32) -> TaskResult {
        let res: i32 = Self::fib(number);
        println!("Fib value: {}", res);
        return Ok(TaskSuccess{});
        // return Err(TaskError{uuid: Uuid::new_v4(), message: "meow".to_string()});
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
