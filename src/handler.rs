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
    router.get("/ping", hello::test,"ping");
    router.get("/hello/redis", hello::redis_handler,"redis");
    router.get("/hello/postgres", hello::postgres_handler,"postgresql");

    router.post("/signup", api::users::signup,"signup");
    router.post("/signin", api::users::signin,"signin");
    router.get("/logout", api::users::logout,"logout");
    router.get("/user/:user_id",api::users::fetch,"user info");
    router.get("/verify_email/:token",api::users::verify_email,"verify email");

    router.get("/post",api::posts::posts_list,"get post list");
    router.post("/post",api::posts::create_post,"create post");
    router.get("/post/:post_id",api::posts::get_post_by_id,"get post by id");

    router.get("/comment",api::comments::get_comments_by_post_id,"get comment");
    router.post("/comment",api::comments::create_comment,"create comment");
    router.get("/error",hello::error_test,"hh");

    let mut chain = Chain::new(router);
    chain.link_before(middleware::Cookies);
    chain.link_before(middleware::Json);
    chain.link_after(middleware::Custom404);
    match Iron::new(chain).http("localhost:3000") {
        Ok(_) => info!("Server start success on 3000"),
        Err(e) => info!("Server start fail {:?}",e),
    }
}
