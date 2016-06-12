pub mod middleware;

use iron::prelude::*;
use iron::middleware::Handler;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use iron::headers::Cookie;
use crypto::{ symmetriccipher, buffer, aes, blockmodes };
use crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };

pub struct Cookies;

impl BeforeMiddleware for Cookies {
    fn before(&self,req:&mut Request) -> IronResult<()> {
        info!("{:?},",req.headers.get_mut::<Cookie>());
        Ok(())
    }
}


//pub fn ex
