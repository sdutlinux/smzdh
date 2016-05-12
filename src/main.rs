#![feature(custom_derive, plugin)]
#![plugin(serde_macros,clippy)]

extern crate iron;
extern crate router;
extern crate serde_json;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::collections::BTreeMap;

fn handler(req: &mut Request) -> IronResult<Response> {

    println!("{}",serde_json::to_string(&BTreeMap::<String,String>::new()).unwrap());
    let query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, query)))
}

fn main() {
    let mut router = Router::new();
    router.get("/", handler);
    router.get("/:query", handler);
    router.post("/hello", handler);

    let _ = Iron::new(router).http("localhost:3000");
}
