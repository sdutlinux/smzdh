use iron::prelude::*;
use router::Router;
use smzdh_commons::databases;
use smzdh_commons::errors::{SError,BError};
use smzdh_commons::headers;
use smzdh_commons::middleware::Cookies;
use rustc_serialize::json::ToJson;
use smzdh_commons::middleware::Json;

pub fn posts_list(req:&mut Request) -> IronResult<Response> {
    let _ = sexpect!(req.extensions.get::<Cookies>(),
                       BError::UserNotLogin);
    let mut pc = pconn!();
    let posts = stry!(databases::post_list(&mut pc,None,None));
    let mut response = headers::JsonResponse::new();
    response.insert("posts",&posts);
    headers::success_json_response(&response)
}

pub fn get_post_by_id(req:&mut Request) -> IronResult<Response> {
    let _ = sexpect!(req.extensions.get::<Cookies>(),
                       BError::UserNotLogin);
    let id = stry!(sexpect!(
        req.extensions.get::<Router>().and_then(|x| x.find("id")),
        SError::ParamsError,"id 不存在。"
    ).parse::<i32>(),
                   SError::ParamsError,"id 格式错误。");
    let mut pc = pconn!();
    let post = sexpect!(stry!(databases::get_post_by_id(&mut pc,id)));
    let mut response = headers::JsonResponse::new();
    response.move_from_btmap(post.to_json());
    headers::success_json_response(&response)
}

pub fn create_post(req:&mut Request) -> IronResult<Response> {
    let uid = sexpect!(req.extensions.get::<Cookies>(),
                       BError::UserNotLogin);
    let json = sexpect!(req.extensions.get::<Json>(),
                        SError::ParamsError,
                        "body 必须是 Json.");
    let object = sexpect!(json.as_object(),
                          SError::ParamsError);
    let title = jget!(object,"title",as_string);
    let content = jget!(object,"content",as_string);
    let mut pc = pconn!();
    stry!(databases::create_post(&mut pc,title,content,*uid));
    headers::success_json_response(&headers::JsonResponse::new())
}
