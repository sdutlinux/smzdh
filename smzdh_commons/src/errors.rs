use iron::status;
use iron::status::Status;
use iron::error::IronError;
use iron::headers::ContentType;
use iron::modifiers::Header;

use std::fmt::Display;
use std::fmt;
use std::error::Error as StdError;

use super::headers;


#[derive(Debug)]
pub enum SError {
    Test,
    InternalServerError,
    ParamsError,
    UserOrPassError,
    UserNotLogin,
    LoginFail,
    Forbidden,
    ResourceNotFound,
}

impl Display for SError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.description())
    }
}

impl StdError for SError {
    fn description(&self) -> &'static str {
        match *self {
            SError::Test => "test error",
            SError::InternalServerError => "服务器内部错误",
            SError::ParamsError => "请求参数错误",
            SError::UserOrPassError => "用户名或者密码错误",
            SError::UserNotLogin => "用户未登陆",
            SError::LoginFail => "登陆失败",
            SError::Forbidden => "未授权",
            SError::ResourceNotFound => "资源不存在",
        }
    }
}

impl SError {
    pub fn to_response(&self,desc:Option<String>) -> (Status, Header<ContentType>, String) {
        let status = match *self {
            SError::InternalServerError => status::InternalServerError,
            SError::Forbidden | SError::UserNotLogin => status::Forbidden
            _ => status::BadRequest,
        };
        let mut response = headers::JsonResponse::new();
        match desc {
            Some(s) => response.set_error(format!("{}:{}",self.description(),&*s)),
            None => response.set_error(self.description()),
        };
        (
            status,headers::json_headers(),
            response.to_json_string(),
        )
    }

    pub fn into_iron_error(self,desc:Option<String>) -> IronError {
        let response = self.to_response(desc);
        IronError::new(self,response)
    }
}
