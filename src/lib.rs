#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
extern crate uuid;
pub extern crate crossbeam;

pub mod api;
pub mod job_request;
pub mod job;
pub mod process;
pub mod queue;
pub mod msg_channel;
pub mod event;
