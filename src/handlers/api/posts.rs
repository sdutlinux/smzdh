use iron::prelude::*;
use router::Router;
use smzdh_commons::databases::{self,UserFlag,VERIFY_EMAIL};
use smzdh_commons::errors::{SError,BError};
use smzdh_commons::headers;
use smzdh_commons::middleware::Cookies;
use rustc_serialize::json::{self,ToJson};
use smzdh_commons::middleware::Json;

pub fn posts_list(req:&mut Request) -> IronResult<Response> {
    let uid = sexpect!(req.extensions.get::<Cookies>(),
                       BError::UserNotLogin);
    pconn!(pc);
    let user = sexpect!(stry!(databases::find_user_by_id(&pc,*uid)));
    check!(UserFlag::from_bits_truncate(user.flags).contains(VERIFY_EMAIL));
    let posts = stry!(databases::post_list(&pc,None,None));
    let mut response = headers::JsonResponse::new();
    response.insert("posts",&posts.into_iter().map(|x|
                                                   {x.into_simple_json()}
    ).collect::<Vec<json::Json>>());
    headers::success_json_response(&response)
}

pub fn get_post_by_id(req:&mut Request) -> IronResult<Response> {
    let uid = sexpect!(req.extensions.get::<Cookies>(),
                       BError::UserNotLogin);
    let id = stry!(
        sexpect!(
            req.extensions.get::<Router>().and_then(|x| x.find("id")),
            SError::ParamsError,"未传入 id 参数。"
        ).parse::<i32>(),
        SError::ParamsError,"id 格式错误。");
    pconn!(pc);
    let user = sexpect!(stry!(databases::find_user_by_id(&pc,*uid)));
    check!(UserFlag::from_bits_truncate(user.flags).contains(VERIFY_EMAIL));
    let post = sexpect!(stry!(databases::get_post_by_id(&pc,id)));
    let mut response = headers::JsonResponse::new();
    response.move_from_btmap(post.to_json());
    headers::success_json_response(&response)
}

pub fn create_post(req:&mut Request) -> IronResult<Response> {
    let uid = sexpect!(req.extensions.get::<Cookies>(),
                       BError::UserNotLogin);
    let object = sexpect!(
        sexpect!(req.extensions.get::<Json>(),
                 SError::ParamsError,
                 "body 必须是 Json.").as_object(),
        SError::ParamsError);
    let title = jget!(object,"title",as_string);
    let content = jget!(object,"content",as_string);
    pconn!(pc);
    let user = sexpect!(stry!(databases::find_user_by_id(&pc,*uid)));
    check!(UserFlag::from_bits_truncate(user.flags).contains(VERIFY_EMAIL));
    stry!(databases::create_post(&pc,title,content,*uid));
    headers::success_json_response(&headers::JsonResponse::new())
}
