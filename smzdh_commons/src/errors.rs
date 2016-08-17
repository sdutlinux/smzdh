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
    pub fn to_response(&self,desc:Option<&str>) -> (Status, Header<ContentType>, String) {
        let status = match *self {
            SmzdhError::InternalServerError => status::InternalServerError,
            SmzdhError::Test | _ => status::BadRequest,
        };
        let mut response = headers::JsonResponse::new();
        match desc {
            Some(s) => response.set_error(format!("{}:{}",self.description(),s)),
            None => response.set_error(self.description()),
        };
        (
            status,headers::json_headers(),
            response.to_json_string(),
        )
    }

    pub fn into_iron_error(self,desc:Option<&str>) -> IronError {
        let response = self.to_response(desc);
        IronError::new(self,response)
    }
}


#[macro_export]
macro_rules! stry {
    ($result:expr) => (stry!($result, $crate::errors::SmzdhError::InternalServerError.into_iron_error(None)));

    ($result:expr, $modifier:expr) => (match $result {
        ::std::result::Result::Ok(val) => val,
        ::std::result::Result::Err(err) => {
            info!("Error case{:?}",err);
            return ::std::result::Result::Err(
                $modifier);
        }
    })
}

/// Unwrap the given `Option` or return a `Ok(Response::new())` with the given
/// modifier. The default modifier is `status::BadRequest`.
#[macro_export]
macro_rules! sexpect {
    ($option:expr) => (sexpect!($option, $crate::errors::SmzdhError::InternalServerError.to_response(None)));
    ($option:expr, $modifier:expr) => (match $option {
        ::std::option::Option::Some(x) => x,
        ::std::option::Option::None =>
            return ::std::result::Result::Ok(::iron::response::Response::with($modifier)),
    })
}
