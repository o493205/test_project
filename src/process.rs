use std::thread;
use std::collections::HashMap;
use job_request::JobRequest;
use job::Job;
use crossbeam::sync::MsQueue;
use std::sync::Arc;
use event::Event;

pub fn start(q: Arc<MsQueue<Event>>) {
    let mut p: Processor = Processor {
        queue: q,
        jobs: HashMap::new(),
    };
    thread::spawn(move || {
        p.process();
    });
}

pub struct Processor {
    queue: Arc<MsQueue<Event>>,
    jobs: HashMap<String, JobRequest>,
}

impl Processor {
    fn process(&mut self) {
        loop {
            match self.queue.try_pop() {
                Some(e) => println!("{:?}", e),
                _ => {}
            }
        }
    }
}
