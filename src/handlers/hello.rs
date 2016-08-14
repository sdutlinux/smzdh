use iron::prelude::*;
use iron::status;
use router::Router;
use smzdh_commons::scredis;
use smzdh_commons::middleware::Connect;
use redis::Commands;
use redis;
use smzdh_commons::headers::{JsonResponse,success_json_response};
use smzdh_commons::errors::SmzdhError;

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

pub fn test(_: &mut Request) -> IronResult<Response> {
    let mut response = JsonResponse::new();
    response.set_result("pong");
    Ok(Response::with(success_json_response(&response)))
}

pub fn error_test(_:&mut Request) -> IronResult<Response> {
    let a:Result<i32,i32> = Err(0);
    let _ = stry!(a,SmzdhError::Test);
    Ok(Response::with((status::Ok, "hello")))
}
