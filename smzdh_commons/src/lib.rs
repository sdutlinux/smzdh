#![feature(plugin)]
#![plugin(clippy)]

extern crate iron;
extern crate crypto;
extern crate rustc_serialize;
#[macro_use]
extern crate log;
extern crate postgres;
extern crate redis;
extern crate router;

pub mod headers;
pub mod utils;
pub mod databases;
pub mod middleware;
pub mod scredis;
mod config;
pub mod errors;





#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
