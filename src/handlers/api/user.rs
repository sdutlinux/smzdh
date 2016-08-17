use iron::prelude::*;
use router::Router;
use rand::{OsRng,Rng};
use smzdh_commons::headers;
use smzdh_commons::utils;
use smzdh_commons::errors::SmzdhError;
use rustc_serialize::json::Json;


use std::io::Read;

pub fn handler(req: &mut Request) -> IronResult<Response> {
    let _ = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    let mut inner = headers::JsonResponse::new();
    inner.insert("username","paomian");
    inner.insert("password","hello");
    let test = headers::JsonResponse::new_with(0,"",&inner);
    headers::success_json_response(&test)
}

pub fn signup(req:&mut Request) -> IronResult<Response> {
    let mut body = String::new();
    let _ = req.body.read_to_string(&mut body);
    let json = stry!(Json::from_str(&*body),
                     SmzdhError::ParamsError.into_iron_error(
                         Some("body 必须是Json.".to_string())
                     ));
    let object = sexpect!(json.as_object(),
                          SmzdhError::ParamsError.to_response(None));
    let username = sexpect!(jget!(object,"username",as_string),
                            SmzdhError::ParamsError.to_response(None));
    let password = sexpect!(jget!(object,"password",as_string),
                            SmzdhError::ParamsError.to_response(None));

    let mut inner = headers::JsonResponse::new();
    inner.insert("username",username);
    inner.insert("passowrd",password);
    let test = headers::JsonResponse::new_with(0,"",&inner);
    headers::success_json_response(&test)
}

pub fn ec(_: &mut Request) -> IronResult<Response> {
    let me = "paomian";
    let mut rng = OsRng::new().ok().unwrap();
    let mut key: [u8; 16] = [0; 16];
    let mut iv: [u8; 16] = [0; 16];
    rng.fill_bytes(&mut key);
    rng.fill_bytes(&mut iv);
    let e = utils::encrypt(me.as_bytes(),&key,&iv).ok().unwrap();
    info!("e:{:?},key:{:?},iv:{:?}",utils::hex(&e),utils::hex(&key),utils::hex(&iv));
    info!("hello");
    headers::success_json_response(&headers::JsonResponse::new())
}
