use crate::constants::REDIS_INSTANCE_NAME;
use redis::Connection;
// use redis::Commands;

pub fn get_redis_client() -> Connection {
    //TODO: Add error handling via ?
    let client = redis::Client::open(format!("redis://{}/", REDIS_INSTANCE_NAME)).unwrap();
    client.get_connection().unwrap()
}
