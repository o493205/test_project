extern crate crossbeam;
extern crate test_project;
use test_project::api;
use test_project::process;
use test_project::event::Event;
use crossbeam::scope;
use crossbeam::sync::MsQueue;
use std::io;
use std::sync::Arc;

fn main() {
    let q: Arc<MsQueue<Event>> = Arc::new(MsQueue::new());
    api::start(q.clone());
    process::start(q.clone());
    print!("Press any key to quit");
    let mut input = String::new();
    io::stdin().read_line(&mut input);
}
