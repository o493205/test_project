use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
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

pub fn start(api_to_processor: Arc<MsQueue<Event>>, processor_to_api: Arc<MsQueue<Event>>) {
    let c: MsgChannel = MsgChannel(api_to_processor, processor_to_api);
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
        let e: Event = Event {
            event_type: EventType::New,
            job: Mutex::new(Some(j)),
        };
        queue.0.push(e);
        uuid
    } else {
        "Unable to validate request.".to_string()
    }
}

#[get("/get_status?<job_id>")]
fn get_status(job_id: &str, queue: State<MsgChannel>) -> String {
    let e: Event = Event {
        event_type: EventType::Status,
        job: Mutex::new(None),
    };
    job_id.to_string()
}

#[error(404)]
fn not_found() -> JSON<Value> {
    JSON(json!({
        "status": "error",
        "reason": "Resource was not found."
    }))
}
