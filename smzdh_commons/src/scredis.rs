use redis::Connection as RedisConn;
use redis;
use super::config;

pub fn redis_conn() -> Result<RedisConn,redis::RedisError> {
    redis::Client::open(config::REDIS).and_then(|c| c.get_connection())
}
