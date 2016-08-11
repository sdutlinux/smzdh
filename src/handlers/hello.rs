use iron::prelude::*;
//use iron::middleware::Handler;
use iron::status;
use router::Router;
use smzdh_commons::scredis;
use smzdh_commons::middleware::Connect;
use redis::Commands;
use redis;

pub fn redis_handler(req: &mut Request) -> IronResult<Response> {
    let query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    let _ = scredis::redis_conn().map(|c| {
        let _:Result<(),redis::RedisError> = c.set("paomian", 42);
    });
    Ok(Response::with((status::Ok, query)))
}

pub fn postgres_handler(req: &mut Request) -> IronResult<Response> {
    let result = req.extensions.get_mut::<Connect>().map(|r| {
        r.get_postgres_conn().map(|c| {
            c.query("SELECT * from pg_user;", &[])
        })
    });
    Ok(Response::with((status::Ok, format!("{:?}",result))))
}
