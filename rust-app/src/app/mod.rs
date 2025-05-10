use crate::constants::{QUEUE_NAME, REDIS_INSTANCE_NAME};
use redis::Commands;

pub fn run_app() {
    if let Ok(number) = fetch_an_integer() {
        println!("IM FUCKING APPPP The number is {}", number);
    } else {
        println!("IM FUCKING APPPP Oh no! Failure!");
    }
}

fn fetch_an_integer() -> redis::RedisResult<isize> {
    let client = redis::Client::open(format!("redis://{}/", REDIS_INSTANCE_NAME))?;
    let mut con = client.get_connection()?;

    // let _: () = con.set("my_key", 42)?;
    // con.get("my_key")
    con.lpush(QUEUE_NAME, 1)
}
