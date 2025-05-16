use std::any;
use std::{thread, time::Duration};
use serde::{de::value, Serialize, Deserialize};
use uuid::Uuid;
use serde_json::{Value, to_value, from_value, from_str, to_string};
use redis::{Client, Commands, Connection};
pub mod errors;
use crate::errors::CeleryError;
use std::collections::HashMap;
use std::any::Any;



pub type TaskResult<T = TaskSuccess, E = TaskError> = Result<T, E>;


pub struct Celery{
    connection: Connection,
    task_registry: HashMap<String, Box<dyn Task<dyn Any>>>,
}

impl Celery {

    pub fn create_celery(broker_url: &str) -> Result<Celery, CeleryError> {
        let client: Client = match redis::Client::open(broker_url) {
            Ok(redis_client) => redis_client,
            Err(e) => {
                return Err(CeleryError::ConnectionFailed(e.to_string()));
            }
        };

        match client.get_connection() {
            Ok(connection) => Ok(Celery { connection: connection, task_registry: HashMap::new()}),
            Err(e) => Err(CeleryError::ConnectionFailed(e.to_string())),
        }
    }

    pub fn run_worker(&mut self) {
        loop {
            let result: Option<String> = self.get_next_task().unwrap();
            match result {
                Some(value) => {
                    // Process the value
                    println!("Task Message: {}", value);


                }
                None => {
                    
                    let sleep_duration: u64 = 1;
                    println!("Queue is empty. Sleeping for {} sec...", sleep_duration);
                    thread::sleep(Duration::from_secs(sleep_duration));
                }
            }
        }
    }


    fn parse_task_name_from_message(task_message: &TaskMessage) {
        return serde_json::from_str(&task_message.name).unwrap();
    }
    fn parse_targs_from_message(task_message: &TaskMessage) -> String {
        return serde_json::from_str(&task_message.targs).unwrap();
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

    pub fn push_task<T>(&mut self, task: Box<dyn Task<T>>, targs: T) -> Result<isize, CeleryError> where T: Serialize + for<'de> Deserialize<'de>
    {
        
        let targs_str: String = task.convert_targs_to_json(targs);
        
        let task_message: TaskMessage = TaskMessage {
            id: Uuid::new_v4(),
            name: task.get_task_name().to_string(),
            targs: targs_str
        };

        let task_message_str: String = serde_json::to_string(&task_message).unwrap();
        println!("{}", &task_message_str);
        let queue_name: &str = "test_v2";
        let result: redis::RedisResult<isize> = self.connection.rpush(queue_name, &task_message_str);
        
        match result {
            Ok(res) => Ok(res),
            Err(e) => Err(CeleryError::TaskPushError(e.to_string())),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TaskMessage {
    id: Uuid,
    name: String,
    targs: String
}

pub struct TaskSuccess {}

pub struct TaskError {
    pub uuid: Uuid,
    pub message: String,
}

pub trait Task<T> 
where 
    T: Serialize + for<'de> Deserialize<'de> 
{

    fn convert_targs_to_json(&self, targs: T) -> String {
        return serde_json::to_string(&targs).unwrap();
    }

    fn convert_json_to_targs(&self, json_value: String) -> T {
        return serde_json::from_str(&json_value).unwrap();
    }

    fn get_task_name(&self) -> &str;
    // fn set_celery(&mut self, celery: Celery);
    // fn get_celery(&self) -> Celery;
    // fn get_targs_type(&self) -> T; 
    // fn run(&self, targs: T) -> TaskResult;



    // // gonna force this into celery, not task
    // fn execute_async(&self, targs: T) -> TaskResult {
        
    //     let res: Result<Value, serde_json::Error> = to_value(targs);
        
    //     let targs: Value;
    //     match res {
    //         Ok(res) => {
    //             targs = res;
    //         }
    //         Err(err) => {
    //             return Err(TaskError{uuid: Uuid::new_v4(), message: err.to_string()});
    //         }
    //     }

    //     let cel = self.get_celery();



    //     Ok(TaskSuccess{})
    // }

}

// macro_rules! targs {
//     ($($key:ident: $value:expr),*) => {{
//         let mut map = HashMap::new();
//         $(
//             map.insert(stringify!($key).to_string(), serde_json::to_value($value).unwrap());
//         )*
//         map
//     }}
// }

// /**
//  * make execute_async take in a Value
//  * each trait needs to specify a struct of type t that from_value is used to return (for the run_task arg), and to_value for the execute_async arg
//  * need to figure out how i want to configure struct vs trait for this - 
//  * 
//  * 
//  * STATIC CELERY CLASS WITH EXECUTE ASYNC
//  * INDIVIDUAL TASK CLASSES THAT IMPL TASK WITH RUN
//  * 
//  */