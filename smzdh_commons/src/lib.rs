#![feature(plugin)]
#![plugin(clippy)]

extern crate iron;
extern crate crypto;
extern crate rustc_serialize;
#[macro_use]
extern crate log;
extern crate postgres;
extern crate redis;
extern crate router;
extern crate rand;
extern crate chrono;

pub mod headers;
pub mod utils;
pub mod databases;
pub mod middleware;
pub mod scredis;
mod config;
pub mod errors;


#[macro_export]
macro_rules! jget {
    ($json:expr,$key:expr,$convert:ident) => (
        sexpect!($json.get($key).and_then(|tmp| {
            tmp.$convert()
        }),
                 $crate::errors::SError::ParamsError,
                 &*format!("{} 必须是一个 {} 类型.",$key,&stringify!($convert)[3..]))
    )
}

#[macro_export]
macro_rules! stry {
    ($result:expr) => (stry!($result, $crate::errors::SError::InternalServerError));

    ($result:expr, $modifier:expr) => (match $result {
        ::std::result::Result::Ok(val) => val,
        ::std::result::Result::Err(err) => {
            info!("Error case{:?}",err);
            return ::std::result::Result::Err(
                $modifier.into_iron_error(None));
        }
    });
    ($result:expr,$modifier:expr,$desc:expr) => (match $result {
        ::std::result::Result::Ok(x) => x,
        ::std::result::Result::Err(err) => {
            info!("Error case{:?}",err);
            return ::std::result::Result::Err(
                $modifier.into_iron_error(
                    ::std::option::Option::Some(
                        ::std::string::String::from($desc)
                    )
                )
            )
        }
    }
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
    });
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
            )
        }
    }
    )
}

#[macro_export]
macro_rules! pconn {
    ($req:expr) => (
        {
            let connect = sexpect!($req.extensions.get_mut::<$crate::middleware::DConnectm>());
            stry!(connect.get_postgres_conn())
        }
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
