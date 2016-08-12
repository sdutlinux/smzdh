use iron::prelude::*;
use iron::status;
use router::Router;
use smzdh_commons::headers;
use rand::{OsRng,Rng};
use smzdh_commons::utils;

pub fn test(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Pong")))
}

pub fn handler(req: &mut Request) -> IronResult<Response> {
    let _ = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    let test = headers::JsonResponse::new_with(0,"",headers::JsonResponse::new());
    /*
    let mut test = headers::Json::new();
    let mut inner = headers::Json::new();
    inner.insert("nihao","paomian");
    inner.insert("wohao","paomian");
    test.insert("hello","world");
    test.insert("world","hhhhh");
    test.insert("yxt",&vec![1,2,3,4]);
    test.insert("dajiahao",&inner.data);
    test.insert("query",query);
     */
    Ok(Response::with(headers::success_json_response(&test)))
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
    Ok(Response::with(headers::success_json_response(&headers::JsonResponse::new())))
}
