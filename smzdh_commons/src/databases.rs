use postgres::error as pe;
use postgres::{Connection, SslMode};
use postgres::rows::Row;
use postgres::types::ToSql;
use super::config;
use super::utils;
use chrono::*;
use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;
use std::marker::Sized;

bitflags! {
    pub flags UserFlag: i64 {
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

trait FromRow {
    fn from_row(row:Row) -> Option<Self> where Self:Sized;
}

trait Has {
    fn has(&self) -> BTreeMap<&str,&ToSql>;
}

macro_rules! db_struct {
    ($(#[$attr:meta])* $s:ident $sdb:ident {$(pub $k:ident:$v:ty),+ }) =>(
        #[derive(Debug)]
        $(#[$attr])*
            pub struct $s {
                $(pub $k:$v),+
            }

        #[derive(Default,Debug)]
        $(#[$attr])*
            pub struct $sdb {
                $(pub $k:Option<$v>),+
            }

        impl FromRow for $s {
            fn from_row(row:Row) -> Option<Self>
                where Self:Sized {
                if row.is_empty() {
                    None
                } else {
                    Some($s {
                        $($k:row.get(stringify!($k))),+
                    })
                }
            }
        }

        impl Has for $sdb {
            fn has(&self) -> BTreeMap<&str,&ToSql> {
                let mut tmp = BTreeMap::new();
                $(if self.$k.is_some() {
                    tmp.insert(stringify!($k),&self.$k as &ToSql);
                });+
                tmp
            }
        }
    );
}

db_struct! {
    User UserDb {
        pub id:i32,
        pub email:String,
        pub username:String,
        pub password:String,
        pub salt:String,
        pub flags:i64,
        pub created:DateTime<Local>
    }
}

impl ToJson for User {
    fn to_json(&self) -> Json {
        let mut tmp = BTreeMap::new();
        let mut flags = vec![];
        if UserFlag::from_bits_truncate(self.flags).contains(VERIFY_EMAIL) {
            flags.push(Json::String(String::from("verify_email")));
        }
        tmp.insert(String::from("id"),Json::I64(self.id as i64));
        tmp.insert(String::from("email"),Json::String(self.email.clone()));
        tmp.insert(String::from("usename"),Json::String(self.username.clone()));
        tmp.insert(String::from("flags"),Json::Array(flags));
        tmp.insert(String::from("created"),Json::String(
            self.created.format("%Y-%m-%d %H:%M:%S").to_string()));
        Json::Object(tmp)
    }
}
db_struct!{
    Post PostDb {
        pub id:i32,
        pub title:String,
        pub content:String,
        pub author:i32,
        pub flags:i64,
        pub created:DateTime<Local>
    }
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

db_struct!{
    Comment CommentDb {
        pub id:i32,
        pub content:String,
        pub author:i32,
        pub post_id:i32,
        pub flags:i64,
        pub created:DateTime<Local>
    }
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

pub fn create_user(conn:&Connection,email:&str,name:&str,pass:&str) -> ::postgres::Result<u64> {
    let (ep,salt) = utils::sha_encrypt(pass);
    conn.execute("INSERT INTO users (email,username,password,salt) VALUES ($1,$2,$3,$4)",
                 &[&email,&name,&ep,&salt])
}

pub fn find_user_by_username(conn:&Connection,name:&str) -> Result<Option<User>,pe::Error> {
    conn.query("SELECT id,email,username,password,salt,flags,created FROM users WHERE username = $1",
               &[&name]).map(|rows| {
                   rows.iter().next().and_then(|row| {
                       User::from_row(row)
                   })
               })
}

pub fn find_user_by_id(conn:&Connection,id:i32) -> Result<Option<User>,pe::Error> {
    conn.query("SELECT id,email,username,password,salt,flags,created FROM users WHERE id = $1",
               &[&id]).map(|rows| {
                   rows.iter().next().and_then(|row| {
                       User::from_row(row)
                   })
               })
}


pub fn update_user_by_uid(conn:&Connection,ud:UserDb,uid:i32) -> ::postgres::Result<u64> {
    let update_data:BTreeMap<&str,&ToSql> = ud.has();
    if update_data.is_empty() { Ok(0) } else {
        let update_field = update_data.keys()
            .filter(|&&x| {
                x.ne("id")
            })
            .enumerate().map(|(index,v)| {
                format!("{}=${}",v,index+1)
            }).collect::<Vec<String>>().join(",");
        let update_value = update_data.values().cloned().collect::<Vec<&ToSql>>();
        conn.execute(&*format!("UPDATE users SET {} where id = {}",update_field,uid),&update_value)
    }
}

pub fn create_post(conn:&Connection,title:&str,content:&str,author:i32)
                   -> ::postgres::Result<u64> {
    conn.execute("INSERT INTO posts (title,content,author) VALUES ($1,$2,$3)",
                 &[&title,&content,&author])
}

pub fn get_post_by_id(conn:&Connection,id:i32) -> Result<Option<Post>,pe::Error> {
    conn.query("SELECT id,title,content,author,flags,created FROM posts WHERE id = $1",
               &[&id]).map(|rows| {
                   rows.iter().next().and_then(|row| {
                       Post::from_row(row)
                   })
               })
}


pub fn post_list(conn:&Connection,skip:Option<i64>,limit:Option<i64>)
                 -> Result<Vec<Post>,pe::Error> {
    conn.query("SELECT id,title,content,author,flags,created FROM posts OFFSET $1 LIMIT $2",
               &[&skip.unwrap_or(0),&limit.unwrap_or(20)]).map(|rows| {
                   rows.iter().filter_map(|row| {
                       Post::from_row(row)
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
                   rows.iter().filter_map(|row| {
                       Comment::from_row(row)
                   }).collect()
               })
}
