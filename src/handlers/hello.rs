use iron::prelude::*;
//use iron::middleware::Handler;
use iron::status;
use router::Router;
use database::utils;
use redis::Commands;

pub fn handler(req: &mut Request) -> IronResult<Response> {
    let query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    //let _:Result<_,> = utils::redis_conn().set("paomian", 42);
    //let _:() = utils::redis_conn().set("paomian", 42).unwrap();
    utils::redis_conn().map(|c| {
        c.set("paomian", 42)
    });
    Ok(Response::with((status::Ok, query)))
}
