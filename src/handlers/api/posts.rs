use iron::prelude::*;
use router::Router;
use redis::Commands;
use rustc_serialize::json::{self,ToJson};

use smzdh_commons::databases::{self,UserFlag,PostFlag,VERIFY_EMAIL,CanCache};
use smzdh_commons::errors::{SError};
use smzdh_commons::headers;
use smzdh_commons::utils;
use smzdh_commons::middleware::Cookies;
use smzdh_commons::middleware::Json;

use std::default::Default;

macro_rules! post_id {
    ($str_post_id:ident,$post_id:ident,$req:expr) => {
        let $str_post_id = stry!(
            $req.extensions.get::<Router>()
                .and_then(|x| x.find("post_id"))
                .ok_or(SError::ParamsError)
                ,"未传入 post_id 参数。"
        );
        let $post_id = stry!(
            $str_post_id.parse::<i32>()
                .map_err(|_| SError::ParamsError)
                ,"post_id 格式错误。"
        );
    }
}

pub fn posts_list(req:&mut Request) -> IronResult<Response> {
    uid!(uid,req);
    let ctg = utils::get_query_params(&req.url,"categroy");
    let mut ctgi:Option<i32> = None;
    if ctg.is_some() {
        ctgi = ctg.and_then(|x| { x.parse::<i32>().ok() });
        check!(ctgi.is_some(),SError::ParamsError,"category 格式错误");
    }
    check_sl!(skip,limit,&req.url);
    pconn!(pc);
    rconn!(rc);
    let user = stry!(
        try_caching!(
            rc,format!("user_{}",uid),
            databases::find_user_by_id(pc,*uid)
        )
    );
    check!(UserFlag::from_bits_truncate(user.flags).contains(VERIFY_EMAIL));
    let posts = stry!(databases::post_list(pc,skip,limit,ctgi));
    let mut response = headers::JsonResponse::new();
    response.insert(
        "posts",
        &posts.into_iter().filter_map(
            |x| {
                if PostFlag::from_bits_truncate(x.flags as i64)
                    .contains(databases::IS_DELETE) {
                        None
                    } else {
                        try_caching!(
                            rc,
                            format!("user_{}",x.author),
                            databases::find_user_by_id(pc,x.author)
                        ).ok().map(|user| {
                            let mut post_obj = x.into_simple_btmap();
                            post_obj.insert(String::from("author"),user.into_simple_json());
                            post_obj.to_json()
                        })
                    }
            }).collect::<Vec<json::Json>>());
    headers::success_json_response(&response)
}

pub fn get_post_by_id(req:&mut Request) -> IronResult<Response> {
    uid!(uid,req);
    post_id!(spid,pid,req);
    pconn!(pc);
    rconn!(rc);
    let user = stry!(
        try_caching!(rc,format!("user_{}",uid),
                     databases::find_user_by_id(pc,*uid))
    );
    check!(UserFlag::from_bits_truncate(user.flags).contains(VERIFY_EMAIL));
    let post = stry!(try_caching!(
        rc,format!("post_{}",pid),
        databases::get_post_by_id(pc,pid),
        3600
    ));
    let author = stry!(
        try_caching!(rc,format!("user_{}",post.author),
                     databases::find_user_by_id(pc,post.author))
    );
    let mut post_obj = post.into_btmap();
    post_obj.insert(String::from("author"),author.into_simple_json());
    let mut response = headers::JsonResponse::new();
    response.move_from_btmap(post_obj.to_json());
    headers::success_json_response(&response)
}

pub fn delete_post_by_id(req:&mut Request) -> IronResult<Response> {
    uid!(uid,req);
    post_id!(spid,pid,req);
    pconn!(pc);
    rconn!(rc);
    let user = stry!(
        try_caching!(rc,format!("user_{}",uid),
                     databases::find_user_by_id(pc,*uid))
    );
    check!(UserFlag::from_bits_truncate(user.flags).contains(VERIFY_EMAIL));
    let post = stry!(databases::get_post_by_id(pc,pid),"Post 不存在。");
    check!(post.author == *uid);
    stry!(databases::update_post_by_id(
        pc,
        databases::PostDb {
            flags:Some(
                {
                    let mut pf = PostFlag::from_bits_truncate(post.flags as i64);
                    pf.insert(databases::IS_DELETE);
                    pf.bits()
                }
            ),..Default::default()
        },post.id));
    stry!(rc.del(format!("post_{}",post.id)));
    headers::sjer()
}

pub fn create_post(req:&mut Request) -> IronResult<Response> {
    uid!(uid,req);
    json!(json,req);
    let req_post = stry!(json.as_object().ok_or(SError::ParamsError)
                         ,"Json 格式因该为 Object。");
    let title = jget!(req_post,"title",as_string);
    let content = jget!(req_post,"content",as_string);
    let category_id = jget!(req_post,"category_id",as_i64) as i32;
    pconn!(pc);
    rconn!(rc);
    stry!(
        try_caching!(
            rc,format!("category_{}",category_id),
            databases::get_category_by_id(pc,category_id)
        ),"Category 不存在。"
    );
    let user = stry!(try_caching!(
        rc,format!("user_{}",uid),
        databases::find_user_by_id(pc,*uid)
    ));
    check!(UserFlag::from_bits_truncate(user.flags).contains(VERIFY_EMAIL));
    stry!(databases::create_post(pc,title,content,*uid,category_id));
    headers::sjer()
}
