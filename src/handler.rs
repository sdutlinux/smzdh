use handlers::{hello,api};
use iron::prelude::*;
use router::Router;
use log::{LogRecord, LogLevelFilter};
use env_logger::LogBuilder;
use smzdh_commons::middleware;
use chrono::offset::local::Local;

use std::env;



fn init_log() {
    let format = |record: &LogRecord| {
        format!("[{}] [{}] [{}] - {}", record.level(),
                Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                record.location().module_path(),
                record.args())
    };

    let mut builder = LogBuilder::new();
    builder.format(format).filter(None, LogLevelFilter::Info);

    if env::var("RUST_LOG").is_ok() {
        builder.parse(&env::var("RUST_LOG").unwrap());
    }

    let _ = builder.init();
}

pub fn run() {
    init_log();
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
