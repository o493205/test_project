use std::thread;
use std::collections::HashMap;
use job_request::JobRequest;
use job::Job;
use crossbeam::sync::MsQueue;
use std::sync::Arc;
use event::Event;
use event::EventType;

pub fn start(api_to_processor: Arc<MsQueue<Event>>, processor_to_api: Arc<MsQueue<Event>>) {
    let mut p: Processor = Processor {
        api_to_processor_channel: api_to_processor,
        processor_to_api_channel: processor_to_api,
        jobs: HashMap::new(),
    };
    thread::spawn(move || {
        p.process();
    });
}

pub struct Processor {
    api_to_processor_channel: Arc<MsQueue<Event>>,
    processor_to_api_channel: Arc<MsQueue<Event>>,
    jobs: HashMap<String, JobRequest>,
}

impl Processor {
    fn process(&mut self) {
        loop {
            match self.api_to_processor_channel.try_pop() {
                Some(e) => {
                    if e.event_type == EventType::Status {
                        self.processor_to_api_channel.push(e);
                    }
                }
                _ => {}
            }
        }
    }
}
