#![feature(plugin)]
#![plugin(clippy)]
#![feature(trace_macros)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate smzdh_commons;

extern crate env_logger;
extern crate iron;
extern crate router;
extern crate rustc_serialize;
extern crate rand;
extern crate redis;
extern crate chrono;

mod handlers;
mod handler;

fn main() {
    handler::run();
}
