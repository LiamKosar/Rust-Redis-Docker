use redis::{Client, Commands, Connection, RedisError};
pub mod errors;
use errors::CeleryError;

/**
 * Returns the Redis broker connection according to the value of broker_url within the directory
 */
pub fn get_broker_connection(broker_url: &str) -> Result<Connection, RedisError> {
    //TODO: Add error handling via ?
    let client: redis::Client = redis::Client::open(broker_url)?;
    client.get_connection()
}

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
