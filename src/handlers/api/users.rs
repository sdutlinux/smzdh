use iron::prelude::*;
use iron::headers::{SetCookie,CookiePair};

use smzdh_commons::headers;
use smzdh_commons::utils;
use smzdh_commons::errors::{SError,BError};
use smzdh_commons::middleware::{Json,Cookies};
use smzdh_commons::databases;

pub fn signup(req:&mut Request) -> IronResult<Response> {
    let json = sexpect!(req.extensions.get::<Json>(),
                        SError::ParamsError,
                        "body必须是Json格式。");
    let object = sexpect!(json.as_object(),
                          SError::ParamsError,
                          "Json 格式错误。");
    let username = jget!(object,"username",as_string);
    let password = jget!(object,"password",as_string);
    let mut postgres_c = pconn!();
    stry!(databases::create_user(&mut postgres_c,username,password));
    headers::success_json_response(&headers::JsonResponse::new())
}

pub fn signin(req:&mut Request) -> IronResult<Response> {
    let json = sexpect!(req.extensions.get::<Json>(),
                        SError::ParamsError,
                        "body 必须是 Json.");
    let object = sexpect!(json.as_object(),
                          SError::ParamsError);
    let username = jget!(object,"username",as_string);
    let password = jget!(object,"password",as_string);
    let mut postgres_c = pconn!();
    let user =  sexpect!(stry!(databases::find_user(&mut postgres_c,username)),
                         SError::UserOrPassError);
    if utils::check_pass(password,&*user.password,&*user.salt) {
        info!("user:{} login success",username);
        headers::success_json_response(&headers::JsonResponse::new()).map(|mut resp| {
            let mut cp = CookiePair::new("smzdh_user".to_string(), format!("{}",user.id));
            cp.max_age = Some(604800);
            resp.headers.set(SetCookie(vec![cp]));
            resp
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
