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
use router::Router;

fn main() {
    match log4rs::init_file("config/log4rs.yaml", Default::default()) {
        Ok(_) => info!("Log4rs start success"),
        Err(e) => println!("{:?}",e),
    }

    let mut router = Router::new();
    //router.get("/test", middleware::sql_test);
    //router.get("/hello/query/:query", handler);
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
