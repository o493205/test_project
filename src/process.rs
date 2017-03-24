use std::thread;
use std::collections::HashMap;
use job_request::JobRequest;
use job::Job;
use crossbeam::sync::MsQueue;
use std::sync::Arc;
use event::Event;
use event::EventType;
use threadpool::ThreadPool;

pub fn start(api_to_processor: Arc<MsQueue<Event>>,
             processor_to_api: Arc<MsQueue<Event>>,
             n_workers: usize,
             n_concurrent_jobs: usize) {
    let mut p: Processor = Processor {
        api_to_processor_channel: api_to_processor,
        processor_to_api_channel: processor_to_api,
        jobs: HashMap::new(),
        max_concurrent_jobs: n_concurrent_jobs,
        pool: ThreadPool::new(n_workers),
    };
    thread::spawn(move || {
        p.process();
    });
}

pub struct Processor {
    api_to_processor_channel: Arc<MsQueue<Event>>,
    processor_to_api_channel: Arc<MsQueue<Event>>,
    jobs: HashMap<String, JobRequest>,
    max_concurrent_jobs: usize,
    pool: ThreadPool,
}

impl Processor {
    fn process(&mut self) {
        loop {
            match self.api_to_processor_channel.try_pop() {
                Some(e) => {
                    print!("Processor {:?}", e);
                    match e.event_type {
                        EventType::New => {
                            if self.jobs.capacity() < self.max_concurrent_jobs {
                                let jr: JobRequest =
                                    e.job.job_request.lock().unwrap().clone().unwrap();
                                self.jobs.insert(e.job.job_id, jr);
                            } else {
                                self.api_to_processor_channel.push(e);
                            }
                        }
                        EventType::Status => self.processor_to_api_channel.push(e),
                        EventType::Cancel => {}
                    }
                }
                _ => {}
            }
        }
    }
}
