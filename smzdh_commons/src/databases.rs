use postgres::error as pe;
use postgres::{Connection, SslMode};
use super::config;
fn create_conn(url:&str) -> Result<Connection,pe::ConnectError> {
    Connection::connect(url,SslMode::None)
}

pub fn conn() -> Result<Connection,pe::ConnectError> {
    create_conn(config::URL)
}

pub fn create_user(conn:&mut ::postgres::Connection,name:&str,pass:&str) -> ::postgres::Result<u64> {
    conn.execute("INSERT INTO users (username,password,salt) VALUES ($1,$2,$3)",
                 &[&name,&pass,&"salt"])
}
