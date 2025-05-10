use redis::Commands;

use crate::constants::QUEUE_NAME;
use crate::our_redis::get_redis_client;

pub fn run_worker() {
    println!("Worker running");
    loop_over_queue();
}

fn loop_over_queue() {
    let mut con: redis::Connection = get_redis_client();
    let mut processed_count = 0;
    print!("AWOOGA");
    loop {
        // Try to pop a value (non-blocking)
        let result: Option<isize> = con.lpop(QUEUE_NAME, None).unwrap();
        match result {
            Some(value) => {
                // Process the value
                println!("Processed value: {}", value);
                let number: isize = fib(value);
                println!("Fib value: {}", number);
                processed_count += 1;
            }
            None => {
                // Queue is empty, exit the loop
                println!("Queue is empty. Processed {} items.", processed_count);
                break;
            }
        }
    }
}

fn fib(number: isize) -> isize {
    if number == 0 {
        return 0;
    } else if number == 1 {
        return 1;
    }
    fib(number - 1) + fib(number - 2)
}
