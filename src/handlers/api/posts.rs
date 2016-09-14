use iron::prelude::*;
use router::Router;
use redis::Commands;
use rustc_serialize::json::{self,ToJson};

use smzdh_commons::databases::{self,UserFlag,PostFlag,VERIFY_EMAIL,CanCache};
use smzdh_commons::errors::{SError,BError};
use smzdh_commons::headers;
use smzdh_commons::utils;
use smzdh_commons::middleware::Cookies;
use smzdh_commons::middleware::Json;

use std::default::Default;

pub fn posts_list(req:&mut Request) -> IronResult<Response> {
    let uid = sexpect!(req.extensions.get::<Cookies>(),
                       BError::UserNotLogin);
    let ctg = utils::get_query_params(&req.url,"categroy");
    let mut ctgi:Option<i32> = None;
    if ctg.is_some() {
        ctgi = ctg.and_then(|x| { x.parse::<i32>().ok() });
        check!(ctgi.is_some(),SError::ParamsError,"category 格式错误");
    }
    check_sl!(skip,limit,&req.url);
    pconn!(pc);
    rconn!(rc);
    let user = try_caching!(
        rc,format!("user_{}",uid),
        sexpect!(stry!(databases::find_user_by_id(&pc,*uid)))
    );
    check!(UserFlag::from_bits_truncate(user.flags).contains(VERIFY_EMAIL));
    let posts = stry!(databases::post_list(&pc,skip,limit,ctgi));
    let mut response = headers::JsonResponse::new();
    response.insert(
        "posts",
        &posts.into_iter().filter_map(
            |x| {
                if PostFlag::from_bits_truncate(x.flags as i64)
                    .contains(databases::IS_DELETE) {
                        None
                    } else {
                        Some(x.into_simple_json())
                    }
            }
        ).collect::<Vec<json::Json>>());
    headers::success_json_response(&response)
}

pub fn get_post_by_id(req:&mut Request) -> IronResult<Response> {
    let uid = sexpect!(req.extensions.get::<Cookies>(),
                       BError::UserNotLogin);
    let id = stry!(
        sexpect!(
            req.extensions.get::<Router>().and_then(|x| x.find("post_id")),
            "未传入 post_id 参数。",g
        ).parse::<i32>(),
        SError::ParamsError,
        "post_id 格式错误。");
    pconn!(pc);
    rconn!(rc);
    let user = try_caching!(rc,format!("user_{}",uid),
                            sexpect!(stry!(databases::find_user_by_id(&pc,*uid))));
    check!(UserFlag::from_bits_truncate(user.flags).contains(VERIFY_EMAIL));
    let post = try_caching!(
        rc,format!("post_{}",id),
        sexpect!(stry!(databases::get_post_by_id(&pc,id)
                       ,BError::ResourceNotFound,"Post 不存在。")),
        3600
    );
    let mut response = headers::JsonResponse::new();
    response.move_from_btmap(post.to_json());
    headers::success_json_response(&response)
}

pub fn delete_post_by_id(req:&mut Request) -> IronResult<Response> {
    let uid = sexpect!(req.extensions.get::<Cookies>(),
                       BError::UserNotLogin);
    let id = stry!(
        sexpect!(
            req.extensions.get::<Router>().and_then(|x| x.find("post_id")),
            "未传入 post_id 参数。",g
        ).parse::<i32>(),
        SError::ParamsError,
        "post_id 格式错误。");
    pconn!(pc);
    rconn!(rc);
    let user = try_caching!(rc,format!("user_{}",uid),
                            sexpect!(stry!(databases::find_user_by_id(&pc,*uid))));
    check!(UserFlag::from_bits_truncate(user.flags).contains(VERIFY_EMAIL));
    let post = sexpect!(stry!(databases::get_post_by_id(&pc,id)
                              ,BError::ResourceNotFound,"Post 不存在。"));
    check!(post.author == *uid);
    stry!(databases::update_post_by_id(
        &pc,
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
    let uid = sexpect!(req.extensions.get::<Cookies>(),
                       BError::UserNotLogin);
    let object = sexpect!(
        sexpect!(req.extensions.get::<Json>(),
                 "body 必须是 Json.",g).as_object(),
        SError::ParamsError);
    let title = jget!(object,"title",as_string);
    let content = jget!(object,"content",as_string);
    let category_id = jget!(object,"category_id",as_i64) as i32;
    pconn!(pc);
    rconn!(rc);
    try_caching!(
        rc,format!("category_{}",category_id),
        sexpect!(stry!(databases::get_category_by_id(&pc,category_id)),
                 BError::ResourceNotFound,"Category 不存在。")
    );
    let user = try_caching!(
        rc,format!("user_{}",uid),
        sexpect!(stry!(databases::find_user_by_id(&pc,*uid)))
    );
    check!(UserFlag::from_bits_truncate(user.flags).contains(VERIFY_EMAIL));
    stry!(databases::create_post(&pc,title,content,*uid,category_id));
    headers::sjer()
}
