use iron::prelude::*;
use smzdh_commons::middleware::Cookies;
use smzdh_commons::middleware::Json;
use smzdh_commons::databases::{self,UserFlag,VERIFY_EMAIL,CanCache};
use smzdh_commons::errors::{SError};
use smzdh_commons::headers;
use smzdh_commons::utils;


pub fn create_comment(req:&mut Request) -> IronResult<Response> {
    uid!(uid,req);
    json!(json,req);
    let req_comment = stry!(json.as_object().ok_or(SError::ParamsError)
                             ,"Comment 格式因该为 Object。");
    let post_id = jget!(req_comment,"post_id",as_i64) as i32;
    let content = jget!(req_comment,"content",as_string);
    pconn!(pc);
    rconn!(rc);
    let user = stry!(
        try_caching!(rc,format!("user_{}",uid),
                     databases::find_user_by_id(pc,*uid))
    );
    stry!(
        try_caching!(
            rc,format!("post_{}",post_id),
            databases::get_post_by_id(pc,post_id)
                ,3600),
        "Post 不存在。"
    );
    check!(UserFlag::from_bits_truncate(user.flags).contains(VERIFY_EMAIL));
    stry!(databases::create_comment(pc,content,*uid,post_id));
    headers::sjer()
}

pub fn get_comments_by_post_id(req:&mut Request) -> IronResult<Response> {
    uid!(uid,req);
    let spid = stry!(utils::get_query_params(&req.url,"post_id")
                     .ok_or(SError::ParamsError)
                     ,"未传入 post_id 参数。");
    let pid = stry!(
        spid.parse::<i32>()
            .map_err(|_| SError::ParamsError)
            ,"post_id 格式错误。"
    );
    check_sl!(skip,limit,&req.url);
    pconn!(pc);
    rconn!(rc);
    let user = stry!(
        try_caching!(rc,format!("user_{}",uid),
                     databases::find_user_by_id(pc,*uid))
    );
    check!(UserFlag::from_bits_truncate(user.flags).contains(VERIFY_EMAIL));
    let comments = stry!(databases::get_comment_by_post_id(pc,pid,skip,limit));
    let mut response = headers::JsonResponse::new();
    response.insert("comments",&comments);
    headers::success_json_response(&response)
}
