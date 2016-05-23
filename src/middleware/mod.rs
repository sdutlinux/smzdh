pub mod middleware;

use iron::prelude::*;
use iron::middleware::Handler;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use iron::headers::Cookie;

pub struct Cookies;

impl BeforeMiddleware for Cookies {
    fn before(&self,req:&mut Request) -> IronResult<()> {
        info!("{:?},",req.headers.get_mut::<Cookie>());
        Ok(())
    }
}
