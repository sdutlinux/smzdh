use handlers::{hello,api};
use iron::prelude::*;
use router::Router;
use log4rs;
use smzdh_commons::middleware;

pub fn run() {
    match log4rs::init_file("config/log4rs.yaml", Default::default()) {
        Ok(_) => info!("Log4rs start success"),
        Err(e) => println!("{:?}",e),
    }

    let mut router = Router::new();
    router.get("/hello/redis", hello::redis_handler);
    router.get("/hello/postgres", hello::postgres_handler);
    router.get("/ping", hello::test);
    router.get("/json", api::user::handler);
    router.get("/ec", api::user::ec);
    router.post("/signup", api::user::signup);
    router.get("/error",hello::error_test);
    let mut chain = Chain::new(router);
    chain.link_before(middleware::Connect);
    chain.link_after(middleware::Custom404);
    match Iron::new(chain).http("localhost:3000") {
        Ok(_) => info!("Server start success on 3000"),
        Err(e) => info!("Server start fail {:?}",e),
    }
}
