use redis::Commands;

fn main() {
    let client = redis::Client::open("redis://redis/").unwrap();
    let mut con = client.get_connection().unwrap();

    let pong: String = con.ping().unwrap();
    println!("Response from Redis: {}", pong);
}
