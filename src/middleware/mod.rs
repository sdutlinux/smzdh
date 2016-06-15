use iron::prelude::*;
use iron::middleware::Handler;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use iron::status;
use iron::headers::Cookie;
use crypto::{ symmetriccipher, buffer, aes, blockmodes };
use crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };
use postgres::{Connection, SslMode};
use postgres;
use database;

pub struct Connect;
pub struct PConnect {
    conn: Option<Result<Connection,postgres::error::ConnectError>>,
}

impl PConnect {
    fn get_conn(& mut self) -> Result<&Connection,&postgres::error::ConnectError> {
        let tmp = match self.conn.as_mut() {
            Some(c) => c.as_ref(),
            None => {
                database::utils::conn().as_ref()
                self.conn = Some();
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

pub fn sql_test(req: &mut Request) -> IronResult<Response> {
    let result = req.extensions.get::<Connect>().map(|r| {
        r.get_conn().map(|c| {
            c.query("SELECT * from pg_user;", &[]);
        })
    });
    Ok(Response::with((status::Ok, format!("{:?}",result))))
}
