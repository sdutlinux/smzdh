use postgres::error as pe;
use postgres::{Connection, SslMode};
use postgres::rows::Row;
use postgres::rows::Iter;
use super::config;
use super::utils;
use chrono::*;
use std::iter::Iterator;

#[derive(RustcEncodable)]
pub struct User {
    pub id:i32,
    pub username:String,
    pub password:String,
    pub salt:String,
    pub created:DateTime<Local>,
}

#[derive(RustcEncodable)]
pub struct Post {
    pub id:i32,
    pub title:String,
    pub content:String,
    pub author:i32,
    pub created:DateTime<Local>,
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

pub fn get_post(conn:&mut Connection,id:i32) -> Result<Option<Post>,pe::Error> {
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


pub fn post_list(conn:&mut Connection,skip:Option<i32>,limit:Option<i32>)
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

/*
pub fn post_list(conn:&mut Connection,skip:Option<i32>,limit:Option<i32>)
                 -> Result<Box<::std::iter::Map<Iter,fn(Row) -> Post>>,pe::Error> {
    conn.query("SELECT id,title,content,author,created FROM posts OFFSET $1 LIMIT $2",
               &[&skip.unwrap_or(0),&limit.unwrap_or(20)]).map(|rows| {
                   Box::new(rows.into_iter().map(|row| {
                       Post {
                           id:row.get(0),
                           title:row.get(1),
                           content:row.get(2),
                           author:row.get(3),
                           created:row.get(4),
                       }
                   }))
               })
}
*/
