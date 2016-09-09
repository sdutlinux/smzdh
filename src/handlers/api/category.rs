use iron::prelude::*;
use smzdh_commons::middleware::Cookies;
use smzdh_commons::middleware::Json;
use smzdh_commons::databases::{self,UserFlag,VERIFY_EMAIL,IS_ADMIN,CanCache};
use smzdh_commons::headers;
use smzdh_commons::errors::{SError,BError};
use rustc_serialize::json::{self};

pub fn create_category(req:&mut Request) -> IronResult<Response> {
    let uid = sexpect!(
        req.extensions.get::<Cookies>(),
        BError::UserNotLogin
    );
    let object = sexpect!(
        sexpect!(
            req.extensions.get::<Json>(),
            "body 必须是 Json",g
        ).as_object()
    );
    pconn!(pc);
    rconn!(rc);
    let name = jget!(object,"name",as_string);
    let desc = jget!(object,"desc",as_string);
    let user = try_caching!(
        rc,format!("user_{}",uid),
        sexpect!(stry!(databases::find_user_by_id(&pc,*uid)))
    );
    let flags = UserFlag::from_bits_truncate(user.flags);
    check!(flags.contains(VERIFY_EMAIL) && flags.contains(IS_ADMIN));
    stry!(databases::create_cagegory(&pc,name,desc));
    headers::sjer()
}

pub fn category_list(req:&mut Request) -> IronResult<Response> {
    let uid = sexpect!(
        req.extensions.get::<Cookies>(),
        BError::UserNotLogin
    );
    check_sl!(skip,limit,&req.url);
    pconn!(pc);
    rconn!(rc);
    let user = try_caching!(
        rc,format!("user_{}",uid),
        sexpect!(stry!(databases::find_user_by_id(&pc,*uid)))
    );
    check!(UserFlag::from_bits_truncate(user.flags).contains(VERIFY_EMAIL));
    let categorys = stry!(databases::get_category_list(&pc,skip,limit));
    let mut response =  headers::JsonResponse::new();
    response.insert(
        "category",
        &categorys.into_iter().map(|x| {
            x.into_json()
        }).collect::<Vec<json::Json>>()
    );
    headers::success_json_response(&response)
}