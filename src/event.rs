use job::Job;

#[derive(Debug)]
pub enum EventType {
    New,
    Query,
    Cancel,
}

#[derive(Debug)]
pub struct Event {
    pub event_type: EventType,
    pub job: Job,
}
