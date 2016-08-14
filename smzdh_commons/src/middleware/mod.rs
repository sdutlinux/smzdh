use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use iron::status;
use iron::headers::Cookie;
use postgres::Connection;
use router::NoRoute;
use redis::Connection as RedisConn;
use redis;
use postgres::error as pe;
use super::databases;
use super::scredis;

pub struct PConnect {
    postgres_conn: Option<Result<Connection,pe::ConnectError>>,
    redis_conn: Option<Result<RedisConn,redis::RedisError>>
}

impl PConnect {
    pub fn get_postgres_conn(& mut self) -> Result<&mut Connection,&mut pe::ConnectError> {
        match self.postgres_conn {
            Some(ref mut c) => c.as_mut(),
            None => {
                self.postgres_conn = Some(databases::conn());
                self.get_postgres_conn()
            },
        }
    }

    pub fn get_redis_conn(&mut self) -> Result<&mut RedisConn, &mut redis::RedisError> {
        match self.redis_conn {
            Some(ref mut c) => c.as_mut(),
            None => {
                self.redis_conn = Some(scredis::redis_conn());
                self.get_redis_conn()
            }
        }
    }

    fn new() -> Self {
        PConnect {
            postgres_conn:None,
            redis_conn:None,
        }
    }
}

pub struct Connect;

impl typemap::Key for Connect { type Value = PConnect ;}

impl BeforeMiddleware for Connect {
    fn before(&self,req:&mut Request) -> IronResult<()> {
        req.extensions.insert::<Connect>(PConnect::new());
        Ok(())
    }
}

struct Cid;

impl typemap::Key for Cid { type Value = i64; }

pub struct Cookies;

impl BeforeMiddleware for Cookies {
    fn before(&self,req:&mut Request) -> IronResult<()> {
        let cookies = req.headers.get_mut::<Cookie>();
        info!("Cookies is {:?}",cookies);
        req.extensions.insert::<Cid>(10);
        Ok(())
    }
}

pub struct Custom404;

impl AfterMiddleware for Custom404 {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        if let Some(_) = err.error.downcast::<NoRoute>() {
            Ok(Response::with((status::NotFound, "Custom 404 response")))
        } else {
            Err(err)
        }
    }
}
