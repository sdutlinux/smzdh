#![feature(custom_derive, plugin)]
#![plugin(serde_macros,clippy)]

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


mod database;
mod handlers;

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

fn test(_: &mut Request) -> IronResult<Response> {
    database::test::test();
    Ok(Response::with((status::Ok, "test")))
}

fn main() {
    match log4rs::init_file("config/log4rs.yaml", Default::default()) {
        Ok(_) => info!("Log4rs start success"),
        Err(e) => println!("{:?}",e),
    }
    let mut router = Router::new();
    router.get("/", handler);
    //router.get("/:query", handler);
    router.get("/hello", test);
    let mut chain = Chain::new(router);
    chain.link_before(handlers::signin_handler::Cookies);
    let _ = Iron::new(chain).http("localhost:3000");
}
