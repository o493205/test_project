extern crate test_project;
use test_project::api;
use test_project::process;
use test_project::event::Event;
use test_project::crossbeam::sync::MsQueue;
use std::io;
use std::sync::Arc;

fn main() {
    let api_to_processor: Arc<MsQueue<Event>> = Arc::new(MsQueue::new());
    let processor_to_api: Arc<MsQueue<Event>> = Arc::new(MsQueue::new());
    api::start(api_to_processor.clone(), processor_to_api.clone());
    process::start(api_to_processor.clone(), processor_to_api.clone(), 4, 0);
    print!("Press any key to quit");
    let mut input = String::new();
    io::stdin().read_line(&mut input);
}
