use postgres::error as pe;
use postgres::{Connection, SslMode};
use super::config;
use super::utils;
use chrono::*;

pub struct User {
    pub id:i32,
    pub username:String,
    pub password:String,
    pub salt:String,
    pub created:DateTime<Local>,
}

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

pub fn find_user(conn:&mut ::postgres::Connection,name:&str) -> Result<Option<User>,pe::Error> {
    conn.query("SELECT id,username,password,salt,created FROM users WHERE username = $1",
               &[&name]).map(|rows| {
                   rows.iter().next().map(|row| {
                       User {
                           id:row.get(0),
                           username:row.get(1),
                           password:row.get(2),
                           salt:row.get(3),
                           created:row.get(4),
                       }
                   })
               })
}
