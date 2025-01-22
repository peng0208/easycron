use chrono::Local;
use cron::Schedule;
use nanoid::nanoid;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::watch::channel;
use tokio::task::JoinHandle;

pub type JobId = String;

#[derive(Clone)]
pub struct JobFunc(Arc<dyn Fn() + Send + Sync>);

pub struct Job {
    pub id: JobId,
    pub name: String,
    pub schedule: Schedule,
    func: JobFunc,
    h: Option<JoinHandle<()>>,
}

impl Job {
    pub fn new<F>(name: &str, expr: &str, func: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        Self::new_with_id(nanoid!().as_str(), name, expr, func)
    }

    pub fn new_with_id<F>(id: &str, name: &str, expr: &str, func: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        let (tx, rx) = channel(false);
        Self {
            id: id.to_string(),
            name: name.to_string(),
            schedule: Schedule::from_str(expr).unwrap(),
            func: JobFunc(Arc::new(func)),
            h: None,
        }
    }

    pub async fn run(&mut self) {
        let schedule = self.schedule.clone();
        let func = self.func.clone();
        self.h = Some(tokio::spawn(async move {
            for upcoming in schedule.upcoming(Local) {
                let dur = (upcoming - Local::now()).num_milliseconds();
                tokio::time::sleep(Duration::from_millis(dur as u64)).await;
                func.0();
            }
        }));
    }

    pub fn stop(&self) {
        if let Some(h) = &self.h {
            h.abort();
        }
    }
}
