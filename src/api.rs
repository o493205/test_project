use std::thread;
use std::sync::Arc;
use rocket;
use rocket::State;
use rocket_contrib::{JSON, Value};
use job_request::JobRequest;
use job::Job;
use uuid::Uuid;
use crossbeam::sync::MsQueue;
use event::Event;
use event::EventType;
use msg_channel::MsgChannel;

pub fn start(q: Arc<MsQueue<Event>>) {
    let c: MsgChannel = MsgChannel(q);
    thread::spawn(|| {
        rocket::ignite()
            .mount("/", routes![submit])
            .catch(errors![not_found])
            .manage(c)
            .launch();
    });
}

#[post("/submit", data = "<request>")]
fn submit(request: JSON<JobRequest>, queue: State<MsgChannel>) -> String {
    let r: JobRequest = request.into_inner();
    let uuid: String = Uuid::new_v4().to_string();
    let j: Job = Job {
        job_id: uuid.clone(),
        job_request: r,
    };
    if j.validate() {
        let e: Event = Event { event_type: EventType::New, job: j };
        queue.0.push(e);
        uuid
    } else {
        "Unable to validate request.".to_string()
    }
}

#[error(404)]
fn not_found() -> JSON<Value> {
    JSON(json!({
        "status": "error",
        "reason": "Resource was not found."
    }))
}
