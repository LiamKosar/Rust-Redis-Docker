use redis::{Client, Commands, Connection};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{thread, time::Duration};
use uuid::Uuid;
pub mod errors;
use crate::errors::CeleryError;
use std::collections::HashMap;

pub type TaskResult<T = TaskSuccess, E = TaskError> = Result<T, E>;

pub struct Celery {
    connection: Connection,
    task_registry: HashMap<String, TaskWrapper>,
}

impl Celery {
    pub fn create_celery(broker_url: &str) -> Result<Celery, CeleryError> {
        let client: Client = match redis::Client::open(broker_url) {
            Ok(redis_client) => redis_client,
            Err(e) => {
                println!("Failed to connect celery to broker");
                return Err(CeleryError::ConnectionFailed(e.to_string()));
            }
        };

        match client.get_connection() {
            Ok(connection) => Ok(Celery {
                connection: connection,
                task_registry: HashMap::new(),
            }),
            Err(e) => Err(CeleryError::ConnectionFailed(e.to_string())),
        }
    }

    pub fn run_worker(&mut self) {
        loop {
            let result: Option<String> = self.get_next_task().unwrap();
            match result {
                Some(message) => {
                    // Process the value
                    println!("Task Message: {}", message);
                    self.process_task(message);
                }
                None => {
                    let sleep_duration: u64 = 2;
                    println!("Queue is empty. Sleeping for {} sec...", sleep_duration);
                    thread::sleep(Duration::from_secs(sleep_duration));
                }
            }
        }
    }

    fn process_task(&self, task_message: String) {
        let task_message: TaskMessage = Self::get_task_message(task_message);
        let task_name: &String = &task_message.name;
        let targs_json: &String = &task_message.targs;
        let task_id: Uuid = task_message.id;
        println!("Processing task_id {}", task_id);

        if let Some(task_wrapper) = self.task_registry.get(task_name) {
            println!("Found task wrapper!");
            (task_wrapper.run_task)(targs_json.to_string()).unwrap();
        } else {
            println!("No task with the name {}", task_name);
        }
    }

    fn get_task_message(task: String) -> TaskMessage {
        let task_message: TaskMessage = serde_json::from_str(&task).unwrap();
        return task_message;
    }

    pub fn get_next_task(&mut self) -> Result<Option<String>, CeleryError> {
        let queue_name: &str = "test_v2";
        let result: redis::RedisResult<Option<String>> = self.connection.lpop(queue_name, None);

        match result {
            Ok(res) => Ok(res),
            Err(e) => Err(CeleryError::TaskFetchError(e.to_string())),
        }
    }

    pub fn push_task<T, TaskImpl>(&mut self, targs: T) -> Result<isize, CeleryError>
    where
        T: Serialize + for<'de> Deserialize<'de>,
        TaskImpl: Task<T>,
    {
        let targs_str: String = TaskImpl::convert_targs_to_json(targs);

        let task_message: TaskMessage = TaskMessage {
            id: Uuid::new_v4(),
            name: TaskImpl::get_task_name(),
            targs: targs_str,
        };

        let task_message_str: String = serde_json::to_string(&task_message).unwrap();
        println!("{}", &task_message_str);
        let queue_name: &str = "test_v2";
        let result: redis::RedisResult<isize> =
            self.connection.rpush(queue_name, &task_message_str);

        match result {
            Ok(res) => Ok(res),
            Err(e) => Err(CeleryError::TaskPushError(e.to_string())),
        }
    }

    pub fn register_task<T, TaskImpl>(&mut self)
    where
        T: Serialize + for<'de> Deserialize<'de>,
        TaskImpl: Task<T>,
    {
        let wrapper: TaskWrapper = TaskWrapper {
            name: TaskImpl::get_task_name(),
            run_task: |json_input| {
                let targs = TaskImpl::convert_json_to_targs(json_input);
                return TaskImpl::run(targs);
            },
        };

        self.task_registry
            .insert(TaskImpl::get_task_name(), wrapper);
    }
}

#[derive(Serialize, Deserialize)]
pub struct TaskMessage {
    id: Uuid,
    name: String,
    targs: String,
}

pub struct TaskSuccess {}

#[derive(Debug)]
pub struct TaskError {
    pub uuid: Uuid,
    pub message: String,
}

struct TaskWrapper {
    name: String,
    run_task: fn(String) -> TaskResult,
}

pub trait Task<T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    fn convert_targs_to_json(targs: T) -> String {
        return serde_json::to_string(&targs).unwrap();
    }

    fn convert_json_to_targs(json_value: String) -> T {
        return serde_json::from_str(&json_value).unwrap();
    }

    fn get_task_name() -> String;

    fn run(targs: T) -> TaskResult;
}
