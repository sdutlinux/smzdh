use iron::prelude::*;
//use iron::middleware::Handler;
use iron::status;
use router::Router;
use database::utils;
use redis::Commands;
use redis;
use std::io::BufReader;

pub fn handler(req: &mut Request) -> IronResult<Response> {
    let query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    utils::redis_conn().map(|c| {
        let _:Result<(),redis::RedisError> = c.set("paomian", 42);
    });
    Ok(Response::with((status::Ok, query)))
}
