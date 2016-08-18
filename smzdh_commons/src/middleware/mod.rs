use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use iron::status;
use iron::headers::{Cookie,ContentType};
use iron::method::Method;
use postgres::Connection;
use router::NoRoute;
use redis::Connection as RedisConn;
use redis;
use postgres::error as pe;
use rustc_serialize::json::Json as RJson;
use iron::mime::{Mime, TopLevel, SubLevel};

use std::io::Read;

use super::databases;
use super::scredis;

pub struct DConnect {
    postgres_conn: Option<Result<Connection,pe::ConnectError>>,
    redis_conn: Option<Result<RedisConn,redis::RedisError>>
}

pub struct DConnectm;

impl DConnect {
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
        DConnect {
            postgres_conn:None,
            redis_conn:None,
        }
    }
}

impl typemap::Key for DConnectm { type Value = DConnect ;}

impl BeforeMiddleware for DConnectm {
    fn before(&self,req:&mut Request) -> IronResult<()> {
        req.extensions.insert::<DConnectm>(DConnect::new());
        Ok(())
    }
}

pub struct Cookies;

impl typemap::Key for Cookies { type Value = i64; }

impl BeforeMiddleware for Cookies {
    fn before(&self,req:&mut Request) -> IronResult<()> {
        let cookies = req.headers.get::<Cookie>();
        info!("Cookies is {:?}",cookies);
        req.extensions.insert::<Cookies>(10);
        Ok(())
    }
}

pub struct Json;

impl typemap::Key for Json { type Value = RJson ;}

impl BeforeMiddleware for Json {
    fn before(&self,req:&mut Request) -> IronResult<()> {
        match req.method {
            Method::Post | Method::Put => {
                let content_type = match req.headers.get::<ContentType>() {
                    Some(ct) => ct,
                    None => {return Ok(());},
                };
                let json = match *content_type {
                    ContentType(Mime(TopLevel::Application, SubLevel::Json, _)) => {
                        let mut body = String::new();
                        let _ = req.body.read_to_string(&mut body);
                        match RJson::from_str(&*body) {
                            Ok(j) => j,
                            Err(e) => {
                                info!("Parse json error raw:{},error:{:?}",body,e);
                                return Err(
                                    super::errors::SmzdhError::ParamsError.into_iron_error(
                                        Some(String::from("Json 格式错误"))
                                    )
                                );
                            }
                        }
                    },
                    _ =>  {return Ok(())},
                };
                req.extensions.insert::<Json>(json);
                Ok(())
            },
            _ => Ok(())
        }
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
