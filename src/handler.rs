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
    router.get("/ping", hello::test);
    router.get("/hello/redis", hello::redis_handler);
    router.get("/hello/postgres", hello::postgres_handler);

    router.post("/signup", api::users::signup);
    router.post("/signin", api::users::signin);
    router.get("/logout", api::users::logout);

    router.get("/post",api::posts::posts_list);
    router.post("/post",api::posts::create_post);
    router.get("/post/:id",api::posts::get_post_by_id);

    router.get("/comment",api::comments::get_comments_by_post_id);
    router.post("/comment",api::comments::create_comment);


    router.get("/error",hello::error_test);
    let mut chain = Chain::new(router);
    chain.link_before(middleware::Cookies);
    chain.link_before(middleware::Json);
    chain.link_after(middleware::Custom404);
    match Iron::new(chain).http("localhost:3000") {
        Ok(_) => info!("Server start success on 3000"),
        Err(e) => info!("Server start fail {:?}",e),
    }
}
