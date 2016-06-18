use iron::prelude::*;
//use iron::middleware::Handler;
use iron::status;
use router::Router;
use database::utils;
use redis::Commands;
use redis;
use p_data;
use protobuf;
use std::io::BufReader;
use protobuf::Message;

pub fn handler(req: &mut Request) -> IronResult<Response> {
    let query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    //let _:Result<_,> = utils::redis_conn().set("paomian", 42);
    //let _:() = utils::redis_conn().set("paomian", 42).unwrap();
    utils::redis_conn().map(|c| {
        let _:Result<(),redis::RedisError> = c.set("paomian", 42);
    });
    Ok(Response::with((status::Ok, query)))
}

pub fn pbuf(req:&mut Request) -> IronResult<Response> {
    let mut me = p_data::user::Me::new();
    me.set_name(String::from("paomian"));
    let mut buf = Vec::<u8>::new();
    {
        let mut cis = protobuf::CodedOutputStream::new(&mut buf as &mut ::std::io::Write);
        me.write_to_with_cached_sizes(&mut cis);
        cis.flush();
    }
    Ok(Response::with((status::Ok, format!("{:?}",buf.clone()))))
}
