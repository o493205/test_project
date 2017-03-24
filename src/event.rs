use std::sync::Mutex;
use job::Job;

#[derive(Debug, PartialEq)]
pub enum EventType {
    New,
    Status,
    Cancel,
}

#[derive(Debug)]
pub struct Event {
    pub event_type: EventType,
    pub job: Job,
}
