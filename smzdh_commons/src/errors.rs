use iron::status;
use iron::status::Status;
use iron::error::IronError;
use iron::headers::ContentType;
use iron::modifiers::Header;

use std::fmt::Display;
use std::fmt;
use std::error::Error;

use super::headers;


#[derive(Debug)]
pub enum SmzdhError {
    Test,
    InternalServerError,
    ParamsError,
}

impl Display for SmzdhError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.description())
    }
}

impl Error for SmzdhError {
    fn description(&self) -> &'static str {
        match *self {
            SmzdhError::Test => "test error",
            SmzdhError::InternalServerError => "服务器内部错误",
            SmzdhError::ParamsError => "请求参数错误",
        }
    }
}

impl SmzdhError {
    pub fn to_response(&self,desc:Option<String>) -> (Status, Header<ContentType>, String) {
        let status = match *self {
            SmzdhError::InternalServerError => status::InternalServerError,
            SmzdhError::Test | _ => status::BadRequest,
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
