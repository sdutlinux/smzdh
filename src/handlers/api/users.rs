use iron::prelude::*;
use iron::headers::{SetCookie,CookiePair};
use router::Router;
use rand::{ Rng, OsRng };
use redis::Commands;
use rustc_serialize::base64::{STANDARD,ToBase64};
use rustc_serialize::json::{ToJson};

use smzdh_commons::headers;
use smzdh_commons::utils::{self,CURRENT_SITE};
use smzdh_commons::email;
use smzdh_commons::errors::{SError};
use smzdh_commons::middleware::{Json,Cookies};
use smzdh_commons::databases::{self,UserFlag,VERIFY_EMAIL,CanCache};

use std::default::Default;
use std::convert::From;

pub fn signup(req:&mut Request) -> IronResult<Response> {
    json!(json,req);
    let req_user = stry!(json.as_object().ok_or(SError::ParamsError)
                         ,"Json 格式因该为 Object。");
    let username = jget!(req_user,"username",as_string);
    let password = jget!(req_user,"password",as_string);
    let email = jget!(req_user,"email",as_string);
    check!(utils::valid_email(email),"email 格式错误。",g);
    pconn!(pc);
    stry!(databases::create_user(pc,email,username,password));
    let user = stry!(databases::find_user_by_username(pc,username));
    rconn!(rc);
    let token = utils::gen_string(8);
    stry!(rc.set_ex(&token,user.id,86400));
    email::send_email(&(format!("{}/verify_email/{}",CURRENT_SITE,token)),&[email]);
    headers::success_json_response(&headers::JsonResponse::new())
}

pub fn signin(req:&mut Request) -> IronResult<Response> {
    if req.extensions.get::<Cookies>().is_some() {
        return headers::success_json_response(&headers::JsonResponse::new());
    }
    json!(json,req);
    let req_user = stry!(json.as_object().ok_or(SError::ParamsError)
                         ,"Json 格式因该为 Object。");
    let email = jget!(req_user,"email",as_string);
    let password = jget!(req_user,"password",as_string);
    pconn!(pc);
    let user =  stry!(databases::find_user_by_email(pc,email)
                      .map_err(|_| SError::UserOrPassError));
    if utils::check_pass(password,&*user.password,&*user.salt) {
        info!("user:{} login success",email);
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
        Ok(Response::with(SError::None.to_response(Some("登陆失败".to_string()))))
    }
}

pub fn fetch(req:&mut Request) -> IronResult<Response> {
    let uid = stry!(req.extensions.get::<Cookies>().ok_or(SError::UserNotLogin));
    let id_str = stry!(req.extensions.get::<Router>()
                       .and_then(|x| x.find("user_id"))
                       .ok_or(SError::ParamsError),
                       "未传入 user_id 参数。");
    self_user!(id,id_str,*uid);
    check!(id==*uid);
    pconn!(pc);
    rconn!(rc);
    let user = stry!(
        try_caching!(rc,format!("user_{}",uid),
                     databases::find_user_by_id(pc,*uid))
    );

    let mut response = headers::JsonResponse::new();
    response.move_from_btmap(user.to_json());
    headers::success_json_response(&response)
}

pub fn verify_email(req:&mut Request) -> IronResult<Response> {
    let token = stry!(
        req.extensions.get::<Router>()
            .and_then(|x| x.find("token"))
            .ok_or(SError::ParamsError),
        "未传入 token 参数。");
    rconn!(rc);
    pconn!(pc);
    let uid:i32 = stry!(rc.get(token).ok().ok_or(SError::ParamsError),"token 无效。");
    let user = stry!(databases::find_user_by_id(pc,uid));
    stry!(databases::update_user_by_uid(
        pc,
        databases::UserDb {
            flags:Some(
                {
                    let mut uf = UserFlag::from_bits_truncate(user.flags as i64);
                    uf.insert(VERIFY_EMAIL);
                    uf.bits()
                }
            ),..Default::default()
        },
        uid));
    stry!(rc.del(format!("user_{}",uid)));
    headers::sjer()
}

pub fn logout(req:&mut Request) -> IronResult<Response> {
    let uid = sexpect!(req.extensions.get::<Cookies>(),
                       SError::UserNotLogin);
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
