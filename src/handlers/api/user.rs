use iron::prelude::*;
use iron::headers::{SetCookie,CookiePair};
use router::Router;

use smzdh_commons::headers;
use smzdh_commons::utils;
use smzdh_commons::errors::SmzdhError;
use smzdh_commons::middleware::Json;
use smzdh_commons::databases;

pub fn handler(req: &mut Request) -> IronResult<Response> {
    let _ = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    let mut inner = headers::JsonResponse::new();
    inner.insert("username","paomian");
    inner.insert("password","hello");
    let test = headers::JsonResponse::new_with(0,"",&inner);
    headers::success_json_response(&test)
}

pub fn signup(req:&mut Request) -> IronResult<Response> {
    let json = sexpect!(req.extensions.get::<Json>(),
                        SmzdhError::ParamsError.to_response(
                            Some("body必须是Json格式。".to_string())
                        )).clone();
    let object = sexpect!(json.as_object(),
                          SmzdhError::ParamsError.to_response(
                              Some("Json 格式错误。".to_string())));
    let username = jget!(object,"username",as_string);
    let password = jget!(object,"password",as_string);
    let postgres_c = pconn!(req);
    stry!(databases::create_user(postgres_c,username,password));
    headers::success_json_response(&headers::JsonResponse::new())
}

pub fn signin(req:&mut Request) -> IronResult<Response> {
    let json = sexpect!(req.extensions.get::<Json>(),
                        SmzdhError::ParamsError.to_response(
                            Some("body 必须是 Json.".to_string())
                        )).clone();
    let object = sexpect!(json.as_object(),
                          SmzdhError::ParamsError.to_response(None));
    let username = jget!(object,"username",as_string);
    let password = jget!(object,"password",as_string);
    let postgres_c = pconn!(req);
    let user =  sexpect!(stry!(databases::find_user(postgres_c,username)),
                         SmzdhError::UserOrPassError.to_response(None));
    if utils::check_pass(password,&*user.password,&*user.salt) {
        info!("user:{} login success",username);
        headers::success_json_response(&headers::JsonResponse::new()).map(|mut resp| {
            resp.headers.set(
                SetCookie(vec![
                    CookiePair::new("smzdh_user".to_string(), format!("{}",user.id))
                ])
            );
            resp
        })
    } else {
        Ok(Response::with(SmzdhError::Test.to_response(Some("登陆失败".to_string()))))
    }
}

pub fn ec(_: &mut Request) -> IronResult<Response> {
    let me = "paomian";
    let (a,b) = utils::encrypt(me);
    info!("e:{},salt:{},check:{}",a,b,utils::check_pass(me,&*a,&*b));
    headers::success_json_response(&headers::JsonResponse::new())
}
