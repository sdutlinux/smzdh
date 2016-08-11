extern crate iron;
extern crate serde_json;
extern crate serde;
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


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}