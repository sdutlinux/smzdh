use postgres::error as pe;
use postgres::{Connection, SslMode};
use super::config;
use super::utils;
use chrono::*;
use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;

bitflags! {
    pub flags UserFlag: i32 {
        const VERIFY_EMAIL       = 0b1,
    }
}

pub fn test() {
    if UserFlag::from_bits_truncate(1).contains(VERIFY_EMAIL) {
        info!("verify email");
    } else {
        info!("not verify email")
    }

}

pub struct User {
    pub id:i32,
    pub email:String,
    pub username:String,
    pub password:String,
    pub salt:String,
    pub flags:i64,
    pub created:DateTime<Local>,
}

impl ToJson for User {
    fn to_json(&self) -> Json {
        let mut tmp = BTreeMap::new();
        tmp.insert(String::from("id"),Json::I64(self.id as i64));
        tmp.insert(String::from("usename"),Json::String(self.username.clone()));
        tmp.insert(String::from("created"),Json::String(
            self.created.format("%Y-%m-%d %H:%M:%S").to_string()));
        Json::Object(tmp)
    }
}

pub struct Post {
    pub id:i32,
    pub title:String,
    pub content:String,
    pub author:i32,
    pub flags:i64,
    pub created:DateTime<Local>,
}

impl ToJson for Post {
    fn to_json(&self) -> Json {
        let mut tmp = BTreeMap::new();
        tmp.insert(String::from("id"),Json::I64(self.id as i64));
        tmp.insert(String::from("title"),Json::String(self.title.clone()));
        tmp.insert(String::from("content"),Json::String(self.content.clone()));
        tmp.insert(String::from("author"),Json::I64(self.author as i64));
        tmp.insert(String::from("created"),Json::String(
            self.created.format("%Y-%m-%d %H:%M:%S").to_string()));
        Json::Object(tmp)
    }
}

impl Post {
    pub fn into_simple_json(self) -> Json {
        let mut tmp = BTreeMap::new();
        tmp.insert(String::from("id"),Json::I64(self.id as i64));
        tmp.insert(String::from("title"),Json::String(self.title));
        tmp.insert(String::from("author"),Json::I64(self.author as i64));
        tmp.insert(String::from("created"),Json::String(
            self.created.format("%Y-%m-%d %H:%M:%S").to_string()));
        Json::Object(tmp)
    }
}
////
pub struct Comment {
    pub id:i32,
    pub content:String,
    pub author:i32,
    pub post_id:i32,
    pub flags:i64,
    pub created:DateTime<Local>,
}

impl ToJson for Comment {
    fn to_json(&self) -> Json {
        let mut tmp = BTreeMap::new();
        tmp.insert(String::from("id"),Json::I64(self.id as i64));
        tmp.insert(String::from("content"),Json::String(self.content.clone()));
        tmp.insert(String::from("author"),Json::I64(self.author as i64));
        tmp.insert(String::from("post_id"),Json::I64(self.id as i64));
        tmp.insert(String::from("created"),Json::String(
            self.created.format("%Y-%m-%d %H:%M:%S").to_string()));
        Json::Object(tmp)
    }
}

fn create_conn(url:&str) -> Result<Connection,pe::ConnectError> {
    Connection::connect(url,SslMode::None)
}

pub fn conn() -> Result<Connection,pe::ConnectError> {
    create_conn(config::URL)
}

pub fn create_user(conn:&Connection,name:&str,pass:&str) -> ::postgres::Result<u64> {
    let (ep,salt) = utils::sha_encrypt(pass);
    conn.execute("INSERT INTO users (username,password,salt) VALUES ($1,$2,$3)",
                 &[&name,&ep,&salt])
}

pub fn find_user(conn:&Connection,name:&str) -> Result<Option<User>,pe::Error> {
    conn.query("SELECT id,email,username,password,salt,flags,created FROM users WHERE username = $1",
               &[&name]).map(|rows| {
                   rows.iter().next().map(|row| {
                       User {
                           id:row.get("id"),
                           email:row.get("email"),
                           username:row.get("username"),
                           password:row.get("password"),
                           salt:row.get("salt"),
                           flags:row.get("flags"),
                           created:row.get("created"),
                       }
                   })
               })
}

pub fn create_post(conn:&Connection,title:&str,content:&str,author:i32)
                   -> ::postgres::Result<u64> {
    conn.execute("INSERT INTO posts (title,content,author) VALUES ($1,$2,$3)",
                 &[&title,&content,&author])
}

pub fn get_post_by_id(conn:&Connection,id:i32) -> Result<Option<Post>,pe::Error> {
    conn.query("SELECT id,title,content,author,flags,created FROM posts WHERE id = $1",
               &[&id]).map(|rows| {
                   rows.iter().next().map(|row| {
                       Post {
                           id:row.get(0),
                           title:row.get(1),
                           content:row.get(2),
                           author:row.get(3),
                           flags:row.get(4),
                           created:row.get(5),
                       }
                   })
               })
}


pub fn post_list(conn:&Connection,skip:Option<i64>,limit:Option<i64>)
                 -> Result<Vec<Post>,pe::Error> {
    conn.query("SELECT id,title,content,author,flags,created FROM posts OFFSET $1 LIMIT $2",
               &[&skip.unwrap_or(0),&limit.unwrap_or(20)]).map(|rows| {
                   rows.iter().map(|row| {
                       Post {
                           id:row.get(0),
                           title:row.get(1),
                           content:row.get(2),
                           author:row.get(3),
                           flags:row.get(4),
                           created:row.get(5),
                       }
                   }).collect()
               })
}

pub fn create_comment(conn:&Connection,content:&str,author:i32,post_id:i32)
                      -> ::postgres::Result<u64> {
conn.execute("INSERT INTO comments (content,author,post_id) VALUES ($1,$2,$3)",
             &[&content,&author,&post_id])
}

pub fn get_comment_by_post_id(conn:&Connection,post_id:i32,skip:Option<i64>
                              ,limit:Option<i64>)
                              -> Result<Vec<Comment>,pe::Error> {
    conn.query("SELECT id,comment,author,post_id,flags,created FROM comments WHERE post_id = $1 OFFSET $2 LIMIT $3",
               &[&post_id,&skip.unwrap_or(0),&limit.unwrap_or(20)]).map(|rows| {
                   rows.iter().map(|row| {
                       Comment {
                           id:row.get(0),
                           content:row.get(1),
                           author:row.get(2),
                           post_id:row.get(3),
                           flags:row.get(4),
                           created:row.get(5)
                       }
                   }).collect()
               })
}
