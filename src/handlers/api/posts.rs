use iron::prelude::*;
use smzdh_commons::databases;
use smzdh_commons::errors::{SError,BError};
use smzdh_commons::headers;
use smzdh_commons::middleware::Cookies;


pub fn posts_list(req:&mut Request) -> IronResult<Response> {
    let uid = sexpect!(req.extensions.get::<Cookies>(),
                       BError::UserNotLogin);
    let mut pc = pconn!();
    let posts = stry!(databases::post_list(&mut pc,None,None),
                      SError::InternalServerError);
    let mut response = headers::JsonResponse::new();
    response.insert("posts",&posts);
    headers::success_json_response(&response)
}
