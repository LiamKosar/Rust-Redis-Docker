use crate::constants::QUEUE_NAME;
use crate::our_redis::get_redis_client;
use redis::Commands;

pub fn run_app() {
    println!("Rust App");
    fetch_an_integer();
}

fn fetch_an_integer() {
    let mut con: redis::Connection = get_redis_client();

    // let _: () = con.set("my_key", 42)?;
    // con.get("my_key")'
    print!("MEOWWW");
    for _ in 0..700 {
        let _: redis::RedisResult<isize> = con.rpush(QUEUE_NAME, 42);
    }
}
