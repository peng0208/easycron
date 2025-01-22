use crate::Job;
use crate::job::JobId;
use derive_builder::Builder;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Default, Builder)]
pub struct Cron {
    #[builder(setter(skip))]
    store: Arc<Mutex<HashMap<JobId, Job>>>,
    #[builder(default)]
    debug: bool,
    #[builder(setter(skip))]
    running: bool,
}
#[derive(Debug)]
enum Event {
    Add(JobId),
    Remove(JobId),
}

impl Cron {
    pub fn list(&self) -> Vec<Job> {
        self.store.lock().unwrap().values().cloned().collect()
    }

    pub async fn add(&mut self, job: Job) {
        {
            let mut store = self.store.lock().unwrap();
            store.insert(job.id.clone(), job.clone());
        }
        self.event(Event::Add(job.id)).await;
    }

    pub async fn remove(&mut self, id: JobId) {
        self.event(Event::Remove(id.clone())).await;
        let mut store = self.store.lock().unwrap();
        store.remove(&id);
    }

    pub async fn run(&mut self) {
        if !self.running {
            for (_, job) in self.store.lock().unwrap().iter_mut() {
                job.run().await;
            }
            self.running = true;
        }
    }

    pub fn stop(&mut self) {
        if self.running {
            for (_, job) in self.store.lock().unwrap().iter_mut() {
                job.stop();
            }
            self.running = false;
        }
    }

    async fn event(&mut self, e: Event) {
        if self.debug {
            println!("Job changed: {:?}", e);
        }

        if !self.running {
            return;
        }

        match e {
            Event::Add(id) => {
                if let Some(job) = self.store.lock().unwrap().get_mut(&id) {
                    job.run().await;
                }
            }
            Event::Remove(id) => {
                if let Some(job) = self.store.lock().unwrap().get_mut(&id) {
                    job.stop();
                }
            }
        }
    }
}
