use postgres::error as pe;
use postgres::{Connection, SslMode};
use super::config;
use super::utils;
use chrono::*;
use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;

pub struct User {
    pub id:i32,
    pub username:String,
    pub password:String,
    pub salt:String,
    pub created:DateTime<Local>,
}

impl ToJson for User {
    fn to_json(&self) -> Json {
        let mut tmp = BTreeMap::new();
        tmp.insert(String::from("id"),Json::I64(self.id as i64));
        tmp.insert(String::from("usename"),Json::String(self.username.clone()));
        tmp.insert(String::from("created"),Json::String(format!{"{}",self.created}));
        Json::Object(tmp)
    }
}

pub struct Post {
    pub id:i32,
    pub title:String,
    pub content:String,
    pub author:i32,
    pub created:DateTime<Local>,
}

impl ToJson for Post {
    fn to_json(&self) -> Json {
        let mut tmp = BTreeMap::new();
        tmp.insert(String::from("id"),Json::I64(self.id as i64));
        tmp.insert(String::from("title"),Json::String(self.title.clone()));
        tmp.insert(String::from("content"),Json::String(self.content.clone()));
        tmp.insert(String::from("author"),Json::I64(self.author as i64));
        tmp.insert(String::from("created"),Json::String(format!{"{}",self.created}));
        Json::Object(tmp)
    }
}

fn create_conn(url:&str) -> Result<Connection,pe::ConnectError> {
    Connection::connect(url,SslMode::None)
}

pub fn conn() -> Result<Connection,pe::ConnectError> {
    create_conn(config::URL)
}

#[macro_export]
macro_rules! pconn {
    () => (
        match $crate::databases::conn() {
            ::std::result::Result::Ok(c) => c,
            ::std::result::Result::Err(e) => {
                info!("{:?}",e);
                return ::std::result::Result::Err(
                    $crate::errors::SError::InternalServerError.into_iron_error(
                        None
                    )
                )
            }
        }
    )
}


pub fn create_user(conn:&mut Connection,name:&str,pass:&str) -> ::postgres::Result<u64> {
    let (ep,salt) = utils::encrypt(pass);
    conn.execute("INSERT INTO users (username,password,salt) VALUES ($1,$2,$3)",
                 &[&name,&ep,&salt])
}

pub fn find_user(conn:&mut Connection,name:&str) -> Result<Option<User>,pe::Error> {
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

//pub fn update_user(conn:&mut ::postgres::Connection,id:i32)

pub fn create_post(conn:&mut Connection,title:&str,content:&str,author:i32)
                   -> ::postgres::Result<u64> {
    conn.execute("INSERT INTO posts (title,content,author) VALUES ($1,$2,$3)",
                 &[&title,&content,&author])
}

pub fn get_post_by_id(conn:&mut Connection,id:i32) -> Result<Option<Post>,pe::Error> {
    conn.query("SELECT id,title,content,author,created FROM posts WHERE id = $1",
               &[&id]).map(|rows| {
                   rows.iter().next().map(|row| {
                       Post {
                           id:row.get(0),
                           title:row.get(1),
                           content:row.get(2),
                           author:row.get(3),
                           created:row.get(4),
                       }
                   })
               })
}


pub fn post_list(conn:&mut Connection,skip:Option<i64>,limit:Option<i64>)
                 -> Result<Vec<Post>,pe::Error> {
    conn.query("SELECT id,title,content,author,created FROM posts OFFSET $1 LIMIT $2",
               &[&skip.unwrap_or(0),&limit.unwrap_or(20)]).map(|rows| {
                   rows.iter().map(|row| {
                       Post {
                           id:row.get(0),
                           title:row.get(1),
                           content:row.get(2),
                           author:row.get(3),
                           created:row.get(4),
                       }
                   }).collect()
               })
}
