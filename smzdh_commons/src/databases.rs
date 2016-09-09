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
use bincode::SizeLimit;
use bincode::rustc_serialize::{encode, decode,EncodingResult,DecodingResult};

fn create_conn(url:&str) -> Result<Connection,pe::ConnectError> {
    Connection::connect(url,SslMode::None)
}

pub fn conn() -> Result<Connection,pe::ConnectError> {
    create_conn(config::URL)
}

bitflags! {
    pub flags UserFlag: i64 {
        const VERIFY_EMAIL       = 0x1,
        const IS_ADMIN           = 0x2,
    }
}

bitflags! {
    pub flags PostFlag:i64 {
        const IS_DELETE         = 0x1,
    }
}

pub fn test() {
    if UserFlag::from_bits_truncate(1).contains(VERIFY_EMAIL) {
        info!("verify email");
    } else {
        info!("not verify email")
    }

}

fn gen_update_field(data:&BTreeMap<&str,&ToSql>) -> String {
    data.keys()
        .filter(|&&x| {
            x.ne("id")
        })
        .enumerate().map(|(index,v)| {
            format!("{}=${}",v,index+1)
        }).collect::<Vec<String>>().join(",")
}


trait FromRow {
    fn from_row(row:Row) -> Option<Self> where Self:Sized;
}

trait Has {
    fn has(&self) -> BTreeMap<&str,&ToSql>;
}

pub trait CanCache {
    fn to_bit(&self) -> EncodingResult<Vec<u8>>;
    fn from_bit(data:&[u8]) -> DecodingResult<Self> where Self:Sized;

}

macro_rules! db_struct {
    ($(#[$attr:meta])* $s:ident $sdb:ident {$(pub $k:ident:$v:ty),+ }) =>(
        #[derive(Debug,RustcDecodable, RustcEncodable)]
        $(#[$attr])*
            pub struct $s {
                $(pub $k:$v),+
            }

        #[derive(Default,Debug,RustcDecodable, RustcEncodable)]
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

        impl CanCache for $s {
            fn to_bit(&self) -> EncodingResult<Vec<u8>> {
                encode(self, SizeLimit::Infinite)
            }

            fn from_bit(data:&[u8]) -> DecodingResult<Self> {
                decode(data)
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
        if UserFlag::from_bits_truncate(self.flags).contains(IS_ADMIN) {
            flags.push(Json::String(String::from("is_admin")));
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
        let update_field = gen_update_field(&update_data);
        let update_value = update_data.values().cloned().collect::<Vec<&ToSql>>();
        conn.execute(&*format!("UPDATE users SET {} where id = {}",update_field,uid),&update_value)
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

pub fn create_post(conn:&Connection,title:&str,content:&str,author:i32,category_id:i32)
                   -> ::postgres::Result<u64> {
    let prepare = try!(conn.prepare_cached(
        "INSERT INTO posts (title,content,author) VALUES ($1,$2,$3) RETURNING id"
    ));
    let result = try!(prepare.query(&[&title,&content,&author]));
    match result.iter().next() {
        Some(x) => {
            let post_id:i32 = x.get("id");
            let ipp = try!(conn.prepare_cached(
                "INSERT INTO post_category (post_id,category_id) VALUES ($1,$2)"
            ));
            ipp.execute(&[&post_id,&category_id])
        },
        None => unreachable!(),
    }
}

pub fn get_post_by_id(conn:&Connection,id:i32) -> Result<Option<Post>,pe::Error> {
    conn.query("SELECT id,title,content,author,flags,created FROM posts WHERE id = $1",
               &[&id]).map(|rows| {
                   rows.iter().next().and_then(|row| {
                       Post::from_row(row)
                   })
               })
}


pub fn post_list(conn:&Connection,skip:i64,limit:i64,category_id:Option<i32>)
                 -> Result<Vec<Post>,pe::Error> {
    match category_id {
        Some(x) => {
            conn.query("SELECT id,content,author,flags,created FROM posts LEFT JOIN post_category ON posts.id = post_category.post_id WHERE post_category.category_id = $1 OFFSET $2 LIMIT $3",
                       &[&x,&skip,&limit])
        },
        None => {
            conn.query("SELECT id,title,content,author,flags,created FROM posts OFFSET $1 LIMIT $2",
                       &[&skip,&limit])
        }
    }.map(|rows| {
        rows.iter().filter_map(|row| {
            Post::from_row(row)
        }).collect()
    })
}

pub fn update_post_by_id(conn:&Connection,pd:PostDb,id:i32) -> ::postgres::Result<u64> {
    let update_data:BTreeMap<&str,&ToSql> = pd.has();
    if update_data.is_empty() { Ok(0) } else {
        let update_field = gen_update_field(&update_data);
        let update_value = update_data.values().cloned().collect::<Vec<&ToSql>>();
        conn.execute(&*format!("UPDATE posts SET {} WHERE id = {}",update_field,id),&update_value)
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

pub fn create_comment(conn:&Connection,content:&str,author:i32,post_id:i32)
                      -> ::postgres::Result<u64> {
    conn.execute("INSERT INTO comments (content,author,post_id) VALUES ($1,$2,$3)",
                 &[&content,&author,&post_id])
}

pub fn get_comment_by_post_id(conn:&Connection,post_id:i32,skip:i64
                              ,limit:i64)
                              -> Result<Vec<Comment>,pe::Error> {
    conn.query("SELECT id,comment,author,post_id,flags,created FROM comments WHERE post_id = $1 OFFSET $2 LIMIT $3",
               &[&post_id,&skip,&limit]).map(|rows| {
                   rows.iter().filter_map(|row| {
                       Comment::from_row(row)
                   }).collect()
               })
}


db_struct!{
    Category CategoryDb {
        pub id:i32,
        pub name:String,
        pub description:String,
        pub flags:i64,
        pub created:DateTime<Local>
    }
}

pub fn create_cagegory(conn:&Connection,name:&str,desc:&str) -> ::postgres::Result<u64> {
    conn.execute("INSERT INTO category (name,description) VALUES ($1,$2)",
                 &[&name,&desc])
}

pub fn get_category_list(conn:&Connection,skip:i64,limit:i64)
                    -> Result<Vec<Category>,pe::Error> {
    conn.query("SELECT id,name,desc,description,created FROM category OFFSET $1 LIMIT $2",
               &[&skip,&limit]).map(|rows| {
                   rows.iter().filter_map(|row| {
                       Category::from_row(row)
                   }).collect()
               })
}

pub fn get_category_by_id(conn:&Connection,id:i32) -> Result<Option<Category>,pe::Error> {
    conn.query("SELECT id,name,desc,flags,created FROM category WHERE id = $1",
               &[&id]).map(|rows| {
                   rows.iter().next().and_then(|row| {
                       Category::from_row(row)
                   })
               })
}

impl ToJson for Category {
    fn to_json(&self) -> Json {
        let mut tmp = BTreeMap::new();
        tmp.insert(String::from("id"),Json::I64(self.id as i64));
        tmp.insert(String::from("name"),Json::String(self.name.clone()));
        tmp.insert(String::from("desc"),Json::String(self.description.clone()));
        tmp.insert(String::from("flags"),Json::I64(self.flags as i64));
        tmp.insert(String::from("created"),Json::String(
            self.created.format("%Y-%m-%d %H:%M:%S").to_string()));
        Json::Object(tmp)
    }
}

impl Category {
    pub fn into_json(self) -> Json {
        let mut tmp = BTreeMap::new();
        tmp.insert(String::from("id"),Json::I64(self.id as i64));
        tmp.insert(String::from("name"),Json::String(self.name));
        tmp.insert(String::from("desc"),Json::String(self.description));
        tmp.insert(String::from("flags"),Json::I64(self.flags as i64));
        tmp.insert(String::from("created"),Json::String(
            self.created.format("%Y-%m-%d %H:%M:%S").to_string()));
        Json::Object(tmp)
    }
}




#[allow(dead_code)]
struct PostCategory {
    pub id:i32,
    pub post_id:i32,
    pub category_id:i32,
    pub created:DateTime<Local>
}
