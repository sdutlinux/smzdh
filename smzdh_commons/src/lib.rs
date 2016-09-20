#![feature(plugin)]
#![plugin(clippy)]

extern crate iron;
extern crate crypto;
extern crate rustc_serialize;
#[macro_use]
extern crate log;
#[macro_use]
extern crate bitflags;
extern crate postgres;
extern crate redis;
extern crate router;
extern crate rand;
extern crate chrono;
extern crate url;
extern crate plugin;
extern crate hyper;
extern crate bincode;
extern crate regex;

thread_local!(
    pub static PC:Result<::postgres::Connection,::postgres::error::ConnectError>
        = ::postgres::Connection::connect(config::URL,::postgres::SslMode::None);
    pub static RC:Result<::redis::Connection,::redis::RedisError>
        = redis::Client::open(config::REDIS)
        .and_then(|c| c.get_connection());
);


#[macro_export]
macro_rules! jget {
    ($json:expr,$key:expr,$convert:ident) => (
        sexpect!(
            $json.get($key).and_then(|tmp| {
                tmp.$convert()
            }),
            $crate::errors::SError::ParamsError,
            &*format!("{} 必须是一个 {} 类型.",$key,&stringify!($convert)[3..]));
    )
}

#[macro_export]
macro_rules! check {
    ($check:expr) => (check!($check,$crate::errors::SError::Forbidden));
    ($check:expr,$error:expr) => (
        if $check {} else {
            return ::std::result::Result::Ok(
                ::iron::response::Response::with(
                    $error.to_response(None)
                ));
        }
    );
    ($check:expr,$error:expr,g) => (check!($check,
                                           $crate::errors::SError::Forbidden,
                                           $error));
    ($check:expr,$error:expr,$desc:expr) => (
        if $check {} else {
            return ::std::result::Result::Ok(
                ::iron::response::Response::with(
                    $error.to_response(
                        ::std::option::Option::Some(
                            ::std::string::String::from($desc)
                        )
                    )
                ));
        }
    );
}

#[macro_export]
macro_rules! stry {
    ($result:expr) => (match $result {
        ::std::result::Result::Ok(val) => val,
        ::std::result::Result::Err(err) => {
            info!("Error case {:?}",err);
            return ::std::result::Result::Err(
                $crate::errors::SError::from(err).into_iron_error(None));
        }
    });
    ($result:expr,$desc:expr) => (match $result {
        ::std::result::Result::Ok(x) => x,
        ::std::result::Result::Err(err) => {
            info!("Error case {:?}",err);
            return ::std::result::Result::Err(
                $crate::errors::SError::from(err).into_iron_error(
                    ::std::option::Option::Some(
                        ::std::string::String::from($desc)
                    )
                )
            );
        }
    };
    )
}

#[macro_export]
macro_rules! sexpect {
    ($option:expr) => (sexpect!($option, $crate::errors::SError::ParamsError));
    ($option:expr, $modifier:expr) => (match $option {
        ::std::option::Option::Some(x) => x,
        ::std::option::Option::None => {
            return ::std::result::Result::Ok(
                ::iron::response::Response::with(
                    $modifier.to_response(None)
                ));
        },
    };);
    ($option:expr,$modifier:expr, g) => (sexpect!($option,
                                                  $crate::errors::SError::ParamsError,
                                                  $modifier));
    ($option:expr,$modifier:expr,$desc:expr) => (match $option {
        ::std::option::Option::Some(x) => x,
        ::std::option::Option::None => {
            return ::std::result::Result::Ok(
                ::iron::response::Response::with(
                    $modifier.to_response(
                        ::std::option::Option::Some(
                            ::std::string::String::from($desc)
                        )
                    )
                )
            );
        }
    };
    )
}


pub fn fuckpg()
              -> &'static Result<::postgres::Connection,::postgres::error::ConnectError> {
    PC.with(|pc| { unsafe { &*(pc as * const _) } })
}

pub fn fuckredis()
                 -> &'static Result<::redis::Connection,::redis::RedisError> {
    RC.with(|rc| { unsafe { &*(rc as * const _) } })
}

#[macro_export]
macro_rules! pconn {
    ($v:ident) => (
        let $v = match *$crate::fuckpg() {
            ::std::result::Result::Ok(ref c) => c,
            ::std::result::Result::Err(ref e) => {
                info!("postgresql conn error {:?}",e);
                return ::std::result::Result::Err(
                    $crate::errors::SError::InternalServerError(Box::new($crate::errors::SError::None)).into_iron_error(
                        None
                    )
                )
            },
        };
    );
}

#[macro_export]
macro_rules! rconn {
    ($v:ident) => (
        let $v = match *$crate::fuckredis() {
            ::std::result::Result::Ok(ref c) => c,
            ::std::result::Result::Err(ref e) => {
                info!("redis conn error {:?}",e);
                return ::std::result::Result::Err(
                    $crate::errors::SError::InternalServerError(
                        Box::new($crate::errors::SError::None)
                    ).into_iron_error(
                        None
                    )
                );
            }
        };
    )
}

#[macro_export]
macro_rules! try_caching {
    ($conn:expr,$key:expr,$data:expr) => (try_caching!($conn,$key,$data,172800));
    ($conn:expr,$key:expr,$data:expr,$ex:expr) => (
        {
            ::redis::Commands::get($conn,$key)
                .map_err($crate::errors::SError::from) //result
                .and_then(|data:Option<Vec<u8>>| {
                    match data {
                        Some(x) => {
                            $crate::databases::CanCache::from_bit(&*x)
                                .map_err($crate::errors::SError::from)
                        },
                        None => {
                            $data
                                .map_err($crate::errors::SError::from)
                                .and_then( |dbdata| {
                                    dbdata.to_bit()
                                        .map_err($crate::errors::SError::from)
                                        .and_then::<(),_>(|edata| {
                                            ::redis::Commands::set_ex($conn,$key,edata,$ex)
                                                .map_err($crate::errors::SError::from)
                                        })
                                        .map(|_| dbdata)
                                })
                        },
                    }
                })
        }
    );
}

pub mod headers;
pub mod utils;
pub mod databases;
pub mod middleware;
pub mod scredis;
pub mod config;
pub mod errors;
pub mod email;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
