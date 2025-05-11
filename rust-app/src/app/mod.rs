use crate::celery::get_broker_connection;
use crate::config::QUEUE_NAME;
use redis::Commands;
mod celery_settings;
use crate::app::celery_settings::BROKER_URL;

pub fn run_app() {
    println!("Rust App");
    fetch_an_integer();
}

fn fetch_an_integer() {
    let mut con: redis::Connection = get_broker_connection(BROKER_URL).unwrap();

    // let _: () = con.set("my_key", 42)?;
    // con.get("my_key")'
    print!("MEOWWW");
    for _ in 0..700 {
        let _: redis::RedisResult<isize> = con.rpush(QUEUE_NAME, 42);
    }
}
