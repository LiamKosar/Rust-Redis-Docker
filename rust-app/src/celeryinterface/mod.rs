use serde::{Serialize, Deserialize};
use celerylib::{Celery, Task,};
use once_cell::sync::Lazy;
use std::sync::Mutex;

static GLOBAL_CELERY: Lazy<Mutex<Celery>> = Lazy::new(|| {
    let broker_url = std::env::var("ONLINE_REDIS_URL").expect("ONLINE_REDIS_URL must be set.");
    Mutex::new(Celery::create_celery(&broker_url).unwrap())
});


pub fn register_task<T, TaskImpl>() where
T: Serialize + for<'de> Deserialize<'de>,
TaskImpl: Task<T> {
    let mut celery = GLOBAL_CELERY.lock().unwrap();
    celery.register_task::<T, TaskImpl>();
}

pub fn push_task<T, TaskImpl>(targs: T) where
T: Serialize + for<'de> Deserialize<'de>,
TaskImpl: Task<T> {
    let mut celery = GLOBAL_CELERY.lock().unwrap();
    celery.push_task::<T, TaskImpl>(targs).unwrap();
}

pub fn run_worker() {
    let mut celery = GLOBAL_CELERY.lock().unwrap();
    celery.run_worker();
}

