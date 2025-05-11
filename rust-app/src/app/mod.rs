use crate::celery::ProducerConnection;
use crate::config::QUEUE_NAME;
mod celery_settings;
use crate::app::celery_settings::BROKER_URL;

pub fn run_app() {
    let mut con: ProducerConnection;

    match ProducerConnection::create_producer_connection(BROKER_URL) {
        Ok(connection) => {
            con = connection;
        }
        Err(e) => {
            println!("There was an error fetching the producer connection: {}", e);
            return;
        }
    }

    push_tasks_to_queue(&mut con)
}

// wanna add retries to this!!!
fn push_task_to_queue(producer_connection: &mut ProducerConnection, number: i32) {
    match producer_connection.push_task(QUEUE_NAME, number) {
        Ok(queue_len) => {
            println!("Added element no. {} to the queue", queue_len);
        }
        Err(e) => {
            println!("The push operation failed for reason: {}", e);
        }
    }
}

fn push_tasks_to_queue(producer_connection: &mut ProducerConnection) {
    for _ in 0..100 {
        push_task_to_queue(producer_connection, 42);
    }
}
