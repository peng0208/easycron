use crate::job::JobId;
use crate::Job;
use derive_builder::Builder;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Default, Builder)]
pub struct Cron {
    #[builder(setter(skip))]
    store: Arc<Mutex<HashMap<JobId, Job>>>,
    #[builder(default)]
    debug: bool,
    #[builder(default)]
    running: bool,
}

impl Cron {
    pub fn list(&self) {
        for (_, job) in self.store.lock().unwrap().iter() {
            println!("id:{} name:{} expr:{}", job.id, job.name, job.schedule);
        }
    }
    pub fn add(&mut self, job: Job) {
        let mut store = self.store.lock().unwrap();
        store.insert(job.id.clone(), job);
    }
    pub fn remove(&mut self, id: JobId) {
        let mut store = self.store.lock().unwrap();
        store.remove(&id);
    }
    pub async fn run(&mut self) {
        if self.running {
            for (_, job) in self.store.lock().unwrap().iter_mut() {
                job.run().await;
            }
            self.running = true;
        }
    }
}
