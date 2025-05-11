use crate::celery::WorkerConnection;
use crate::config::QUEUE_NAME;
mod celery_settings;
use crate::worker::celery_settings::BROKER_URL;

pub fn run_worker() {
    println!("Worker running");

    let mut con: WorkerConnection;

    match WorkerConnection::create_worker_connection(BROKER_URL) {
        Ok(connection) => {
            con = connection;
        }
        Err(e) => {
            println!("There was an error fetching the worker connection: {}", e);
            return;
        }
    }

    loop_over_queue(&mut con);
}

fn get_next_task(worker_connection: &mut WorkerConnection) -> Option<isize> {
    match worker_connection.get_next_task(QUEUE_NAME) {
        Ok(task) => task,
        Err(e) => {
            println!("There was an error fetching the task from the queue: {}", e);
            None
        }
    }
}

fn loop_over_queue(worker_connection: &mut WorkerConnection) {
    let mut processed_count = 0;
    print!("AWOOGA");
    loop {
        // Try to pop a value (non-blocking)
        let result: Option<isize> = get_next_task(worker_connection);
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
