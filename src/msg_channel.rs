use std::sync::Arc;
use crossbeam::sync::MsQueue;
use event::Event;

pub struct MsgChannel(pub Arc<MsQueue<Event>>, pub Arc<MsQueue<Event>>);
