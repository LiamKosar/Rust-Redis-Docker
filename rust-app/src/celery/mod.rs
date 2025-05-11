use redis::{Client, Commands, Connection};
pub mod errors;
use errors::CeleryError;

pub struct WorkerConnection {
    connection: Connection,
}

impl WorkerConnection {
    pub fn create_worker_connection(broker_url: &str) -> Result<Self, CeleryError> {
        let client: Client = match redis::Client::open(broker_url) {
            Ok(redis_client) => redis_client,
            Err(e) => {
                return Err(CeleryError::ConnectionFailed(e.to_string()));
            }
        };

        match client.get_connection() {
            Ok(connection) => Ok(WorkerConnection { connection }),
            Err(e) => Err(CeleryError::ConnectionFailed(e.to_string())),
        }
    }

    pub fn get_next_task(&mut self, queue_name: &str) -> Result<Option<isize>, CeleryError> {
        let result: redis::RedisResult<Option<isize>> = self.connection.lpop(queue_name, None);

        match result {
            Ok(res) => Ok(res),
            Err(e) => Err(CeleryError::TaskFetchError(e.to_string())),
        }
    }
}

pub struct ProducerConnection {
    connection: Connection,
}

impl ProducerConnection {
    pub fn create_producer_connection(broker_url: &str) -> Result<Self, CeleryError> {
        let client: Client = match redis::Client::open(broker_url) {
            Ok(redis_client) => redis_client,
            Err(e) => {
                return Err(CeleryError::ConnectionFailed(e.to_string()));
            }
        };

        match client.get_connection() {
            Ok(connection) => Ok(ProducerConnection { connection }),
            Err(e) => Err(CeleryError::ConnectionFailed(e.to_string())),
        }
    }

    pub fn push_task(&mut self, queue_name: &str, number: i32) -> Result<isize, CeleryError> {
        let result: redis::RedisResult<isize> = self.connection.rpush(queue_name, number);

        match result {
            Ok(res) => Ok(res),
            Err(e) => Err(CeleryError::TaskPushError(e.to_string())),
        }
    }
}
