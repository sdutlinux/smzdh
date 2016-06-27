use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use iron::status;
use iron::headers::Cookie;
//use crypto::{ symmetriccipher, buffer, aes, blockmodes };
//use crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };
use postgres::Connection;
use router::NoRoute;
use postgres::error as pe;
use database;

pub struct Connect;

pub struct PConnect {
    conn: Option<Result<Connection,pe::ConnectError>>,
}

impl PConnect {
    pub fn get_conn(& mut self) -> Result<&mut Connection,&mut pe::ConnectError> {
        match self.conn {
            Some(ref mut c) => c.as_mut(),
            None => {
                self.conn = Some(database::utils::conn());
                self.get_conn()
            },
        }
    }
}

impl typemap::Key for Connect { type Value = PConnect ;}

impl BeforeMiddleware for Connect {
    fn before(&self,req:&mut Request) -> IronResult<()> {
        req.extensions.insert::<Connect>(PConnect{conn:None});
        Ok(())
    }
}

struct Cid;

impl typemap::Key for Cid { type Value = i64; }

pub struct Cookies;

impl BeforeMiddleware for Cookies {
    fn before(&self,req:&mut Request) -> IronResult<()> {
        let cookies = req.headers.get_mut::<Cookie>();
        info!("Cookies is {:?}",cookies);
        req.extensions.insert::<Cid>(10);
        Ok(())
    }
}

pub struct Custom404;

impl AfterMiddleware for Custom404 {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        info!("Hitting custom 404 middleware");

        if let Some(_) = err.error.downcast::<NoRoute>() {
            Ok(Response::with((status::NotFound, "Custom 404 response")))
        } else {
            Err(err)
        }
    }
}
