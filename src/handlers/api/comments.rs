use iron::prelude::*;
use smzdh_commons::middleware::Cookies;
use smzdh_commons::middleware::Json;
use smzdh_commons::databases::{self,UserFlag,VERIFY_EMAIL,CanCache};
use smzdh_commons::errors::{SError,BError};
use smzdh_commons::headers;
use smzdh_commons::utils;


pub fn create_comment(req:&mut Request) -> IronResult<Response> {
    let uid = sexpect!(
        req.extensions.get::<Cookies>(),
        BError::UserNotLogin);
    let object = sexpect!(
        sexpect!(req.extensions.get::<Json>(),
                 SError::ParamsError,
                 "body 必须是 Json.").as_object(),
        SError::ParamsError);
    let post_id = jget!(object,"post_id",as_i64) as i32;
    let content = jget!(object,"content",as_string);
    pconn!(pc);
    rconn!(rc);
    let user = try_caching!(rc,format!("user_{}",uid),
                            sexpect!(stry!(databases::find_user_by_id(pc,*uid))));
    try_caching!(
        rc,format!("post_{}",post_id),
        sexpect!(stry!(databases::get_post_by_id(pc,post_id)),
                 BError::ResourceNotFound,"Post 不存在。"),3600
    );
    check!(UserFlag::from_bits_truncate(user.flags).contains(VERIFY_EMAIL));
    stry!(databases::create_comment(pc,content,*uid,post_id));
    headers::sjer()
}

pub fn get_comments_by_post_id(req:&mut Request) -> IronResult<Response> {
    let uid = sexpect!(req.extensions.get::<Cookies>(),
                       BError::UserNotLogin);
    let post_id = stry!(
        sexpect!(utils::get_query_params(&req.url,"post_id"))
            .parse::<i32>(),
        SError::ParamsError,"post_id 应该为一个数字");
    check_sl!(skip,limit,&req.url);
    pconn!(pc);
    rconn!(rc);
    let user = try_caching!(rc,format!("user_{}",uid),
                            sexpect!(stry!(databases::find_user_by_id(pc,*uid))));
    check!(UserFlag::from_bits_truncate(user.flags).contains(VERIFY_EMAIL));
    let comments = stry!(databases::get_comment_by_post_id(pc,post_id,skip,limit));
    let mut response = headers::JsonResponse::new();
    response.insert("comments",&comments);
    headers::success_json_response(&response)
}
