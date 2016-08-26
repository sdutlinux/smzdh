use redis::Connection as RedisConn;
use redis;
use super::config;

pub fn redis_conn() -> Result<RedisConn,redis::RedisError> {
    redis::Client::open(config::REDIS).and_then(|c| c.get_connection())
}

#[macro_export]
macro_rules! rconn {
    ($v:ident) => (
        let $v = match $crate::scredis::redis_conn() {
            ::std::result::Result::Ok(c) => c,
            ::std::result::Result::Err(e) => {
                info!("{:?}",e);
                return ::std::result::Result::Err(
                    $crate::errors::SError::InternalServerError.into_iron_error(
                        None
                    )
                );
            }
        };
    )
}
