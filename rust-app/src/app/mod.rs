use crate::constants::QUEUE_NAME;
use crate::our_redis::get_redis_client;
use redis::Commands;

pub fn run_app() {
    if let Ok(number) = fetch_an_integer() {
        println!("IM FUCKING APPPP The number is {}", number);
    } else {
        println!("IM FUCKING APPPP Oh no! Failure!");
    }
}

fn fetch_an_integer() -> redis::RedisResult<isize> {
    let mut con: redis::Connection = get_redis_client();

    // let _: () = con.set("my_key", 42)?;
    // con.get("my_key")'
    print!("MEOWWW");
    let _: redis::RedisResult<isize> = con.rpush(QUEUE_NAME, 100);
    let _: redis::RedisResult<isize> = con.rpush(QUEUE_NAME, 101);
    let _: redis::RedisResult<isize> = con.rpush(QUEUE_NAME, 102);
    con.rpush(QUEUE_NAME, 103)
}
