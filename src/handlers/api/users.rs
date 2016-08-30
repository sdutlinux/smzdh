use iron::prelude::*;
use iron::headers::{SetCookie,CookiePair};
use rand::{ Rng, OsRng };
use redis::Commands;
use rustc_serialize::base64::{STANDARD,ToBase64};

use smzdh_commons::headers;
use smzdh_commons::utils;
use smzdh_commons::errors::{SError,BError};
use smzdh_commons::middleware::{Json,Cookies};
use smzdh_commons::databases;

pub fn signup(req:&mut Request) -> IronResult<Response> {
    let object = sexpect!(
        sexpect!(req.extensions.get::<Json>(),
                 "body 必须是 Json.",g).as_object(),
        "Json 格式错误。",g);
    let username = jget!(object,"username",as_string);
    let password = jget!(object,"password",as_string);
    let email = jget!(object,"email",as_string);
    pconn!(pc);
    stry!(databases::create_user(&pc,email,username,password));
    headers::success_json_response(&headers::JsonResponse::new())
}

pub fn signin(req:&mut Request) -> IronResult<Response> {
    if req.extensions.get::<Cookies>().is_some() {
        return headers::success_json_response(&headers::JsonResponse::new());
    }
    let object = sexpect!(
        sexpect!(req.extensions.get::<Json>(),
                 "body 必须是 Json.",g).as_object(),
        "Json 格式错误。",g);
    let username = jget!(object,"username",as_string);
    let password = jget!(object,"password",as_string);
    pconn!(pc);
    let user =  sexpect!(stry!(databases::find_user_by_username(&pc,username)),
                         SError::UserOrPassError);
    if utils::check_pass(password,&*user.password,&*user.salt) {
        info!("user:{} login success",username);
        let mut rng = OsRng::new().ok().unwrap();
        let mut r = [0;16];
        rng.fill_bytes(&mut r);
        headers::success_json_response(&headers::JsonResponse::new()).and_then(|mut resp| {
            let ed = stry!(utils::encrypt_cookie(&r,&*user.salt));
            let es = ed.to_base64(STANDARD);
            rconn!(rc);
            stry!(rc.set_ex(r.to_base64(STANDARD),user.id,604800));
            let mut cp = CookiePair::new("smzdh_user".to_string(),
                                         es);
            cp.max_age = Some(604800);
            resp.headers.set(SetCookie(vec![cp]));
            Ok(resp)
        })
    } else {
        Ok(Response::with(SError::Test.to_response(Some("登陆失败".to_string()))))
    }
}

pub fn logout(req:&mut Request) -> IronResult<Response> {
    let uid = sexpect!(req.extensions.get::<Cookies>(),
                       BError::UserNotLogin);
    info!("User uid:{} logout!",uid);
    headers::success_json_response(&headers::JsonResponse::new()).map(|mut resp| {

        let mut cp = CookiePair::new("smzdh_user".to_string(),"invalidate".to_string());
        cp.max_age = Some(0);
        resp.headers.set(SetCookie(vec![
            cp
        ]));
        resp
    })
}
