use iron::prelude::*;
//use iron::middleware::Handler;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use iron::headers::Cookie;

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

impl AfterMiddleware for Cookies {
    fn after(&self, _: &mut Request, res: Response) -> IronResult<Response> {
        Ok(res)
    }
}
