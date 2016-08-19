use postgres::error as pe;
use postgres::{Connection, SslMode};
use postgres::types::FromSql;
use super::config;
use super::utils;
use std::collections::BTreeMap;

fn create_conn(url:&str) -> Result<Connection,pe::ConnectError> {
    Connection::connect(url,SslMode::None)
}

pub fn conn() -> Result<Connection,pe::ConnectError> {
    create_conn(config::URL)
}

pub fn create_user(conn:&mut ::postgres::Connection,name:&str,pass:&str) -> ::postgres::Result<u64> {
    let (ep,salt) = utils::encrypt(pass);
    conn.execute("INSERT INTO users (username,password,salt) VALUES ($1,$2,$3)",
                 &[&name,&ep,&salt])
}

pub fn find_user(conn:&mut ::postgres::Connection,name:&str) -> BTreeMap<String,Box<FromSql>> {
    BTreeMap::new()
}
