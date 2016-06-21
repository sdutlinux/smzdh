#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate nom;

extern crate log4rs;
extern crate iron;
extern crate router;
extern crate serde_json;
extern crate postgres;
extern crate redis;
extern crate crypto;
extern crate serde;
extern crate smzdm_commons;

mod database;
mod handlers;
mod middleware;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::collections::BTreeMap;
use serde_json::value::Value;
use smzdm_commons::headers;

fn handler(req: &mut Request) -> IronResult<Response> {
    let query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    let mut test = BTreeMap::<String,Value>::new();
    test.insert(String::from("test"),Value::Bool(true));
    Ok(Response::with((status::Ok,headers::json_headers(),serde_json::to_string(&test)
                       .unwrap_or(String::from("{}")))))
}

fn main() {
    match log4rs::init_file("config/log4rs.yaml", Default::default()) {
        Ok(_) => info!("Log4rs start success"),
        Err(e) => println!("{:?}",e),
    }

    let mut router = Router::new();
    router.get("/test", middleware::sql_test);
    router.get("/hello/query/:query", handler);
    router.get("/hello/redis", handlers::hello::handler);
    router.get("/ping", handlers::api::user::test);
    let mut chain = Chain::new(router);
    chain.link_before(middleware::Connect);
    chain.link_after(middleware::Custom404);
    match Iron::new(chain).http("localhost:3000") {
        Ok(_) => info!("Server start success on 3000"),
        Err(e) => info!("Server start fail {:?}",e),
    }
}
