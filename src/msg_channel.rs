use std::sync::Arc;
use crossbeam::sync::MsQueue;
use event::Event;

pub struct MsgChannel {
    pub chan: Arc<MsQueue<Event>>,
}
