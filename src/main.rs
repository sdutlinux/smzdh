#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

extern crate log4rs;
extern crate iron;
extern crate router;
extern crate serde_json;
extern crate postgres;
extern crate redis;
extern crate crypto;


mod database;
mod handlers;
mod middleware;

use iron::prelude::*;
use iron::status;
use router::Router;
//use std::collections::BTreeMap;

fn handler(req: &mut Request) -> IronResult<Response> {
    info!("Some thing {:?}",req.extensions.len());
    info!("{:?} \n {:?}",req,req.headers);
    //println!("{}",serde_json::to_string(&BTreeMap::<String,String>::new()).unwrap());
    let query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, query)))
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
    match Iron::new(chain).http("localhost:3000") {
        Ok(_) => info!("Server start success on 3000"),
        Err(e) => info!("Server start fail {:?}",e),
    }
}
