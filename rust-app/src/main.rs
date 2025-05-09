use redis::Commands;

fn main() {
    let hi;
    if let Ok(number) = fetch_an_integer() {
        println!("The number is {}", number);
        hi = number;
    }
    else {
        println!("Oh no! Failure!");
        hi = -1;
    }

    println!("The number is {}", hi);
}

fn fetch_an_integer() -> redis::RedisResult<isize> {
    let client = redis::Client::open("redis://redis/")?;
    let mut con = client.get_connection()?;

    let _: () = con.set("my_key", 42)?;
    con.get("my_key")
}