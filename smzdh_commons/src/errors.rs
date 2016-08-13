use iron::status;
use iron::status::Status;
use iron::error::IronError;

use std::fmt::Display;
use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum SmzdhError {
    Test,
}

impl Display for SmzdhError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SmzdhError::Test => write!(f,"{}","test error")
        }
    }
}

impl Error for SmzdhError {
    fn description(&self) -> &str {
        match *self {
            SmzdhError::Test => "test error",
        }
    }
}

impl SmzdhError {
    pub fn to_response(&self) -> (Status,String) {
        match *self {
            SmzdhError::Test => (status::BadRequest,String::from("test"))
        }
    }

    pub fn into_iron_error(self) -> IronError {
        let response = self.to_response();
        IronError::new(self,response)
    }
}
