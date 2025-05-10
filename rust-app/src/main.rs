use redis::Commands;
use std::env;
mod constants;
use crate::constants::{APP_RUN_MODE, REDIS_INSTANCE_NAME, WORKER_RUN_MODE};

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

fn run_app() {
    if let Ok(number) = fetch_an_integer() {
        println!("IM FUCKING APPPP The number is {}", number);
    } else {
        println!("IM FUCKING APPPP Oh no! Failure!");
    }
}

fn run_worker() {
    println!("IM A FUCKING WORKER")
}

fn fetch_an_integer() -> redis::RedisResult<isize> {
    let client = redis::Client::open(format!("redis://{}/", REDIS_INSTANCE_NAME))?;
    let mut con = client.get_connection()?;

    let _: () = con.set("my_key", 42)?;
    con.get("my_key")
}
