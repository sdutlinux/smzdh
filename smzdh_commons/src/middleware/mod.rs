use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use iron::status;
use iron::headers::{Cookie,ContentType};
use iron::method::Method;
use router::NoRoute;
use rustc_serialize::json::Json as RJson;
use iron::mime::{Mime, TopLevel, SubLevel};
use rustc_serialize::base64::{FromBase64,STANDARD,ToBase64};
use std::io::Read;
use redis::Commands;

use super::utils;
use super::errors::{SError};

pub struct Cookies;

impl typemap::Key for Cookies { type Value = i32; }

impl BeforeMiddleware for Cookies {
    fn before(&self,req:&mut Request) -> IronResult<()> {
        let uid:i32;
        {
            let smzdh_user = match req.headers.get::<Cookie>().and_then(|cookies| {
                cookies.iter().find(|cookie| {
                    &*cookie.name == "smzdh_user"
                })
            }) {
                Some(x) => x,
                None => {return Ok(());},
            };
            let ebu = match smzdh_user.value.from_base64() {
                Ok(s) => s,
                Err(e) => {
                    info!("from base64 fail {:?}",e);
                    return Ok(());
                },
            };
            let bu = stry!(utils::decrypt_cookie(&ebu)
                           .map_err(|_| SError::UserNotLogin))
                [0..16].to_base64(STANDARD);
            rconn!(rc);
            match rc.get(bu) {
                Ok(x) => {uid = x},
                Err(e) => {
                    info!("get cookie error:{:}",e);
                    return Ok(());
                }
            }
        }
        req.extensions.insert::<Cookies>(uid);
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
                                    super::errors::SError::ParamsError.into_iron_error(
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
