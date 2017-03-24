use std::collections::VecDeque;
use std::sync::Mutex;
use job::Job;
use lazy_static;

lazy_static! {
    pub static ref QUEUE: Mutex<VecDeque<Job>> = Mutex::new(VecDeque::new());
}
