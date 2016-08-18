use crypto::digest::Digest;
use crypto::sha2::Sha512;
use rustc_serialize::hex::ToHex;
use rand::{ Rng, OsRng };

pub fn encrypt(pass:&str) -> (String,String) {
    let mut rng = OsRng::new().ok().unwrap();
    let mut salt = [0;32];
    rng.fill_bytes(&mut salt);
    let mut hasher = Sha512::new();
    let hex_salt = hex(&salt);
    let e = [&*hex_salt,pass].concat();
    hasher.input_str(&*e);
    let ep = hasher.result_str();
    (ep,hex_salt)
}

pub fn hex(data:&[u8]) -> String {
    data.to_hex()
}

pub fn check_pass(p:&str,ep:&str,salt:&str) -> bool {
    let mut hasher = Sha512::new();
    hasher.input_str(&*[salt,p].concat());
    &hasher.result_str() == ep
}

#[macro_export]
macro_rules! jget {
    ($json:expr,$key:expr,$convert:ident) => (
        sexpect!($json.get($key).and_then(|tmp| {
            tmp.$convert()
        }),$crate::errors::SmzdhError::ParamsError.to_response(
            Some(format!("{} 必须是一个 {} 类型.",$key,&stringify!($convert)[3..]))))
    )
}

#[macro_export]
macro_rules! stry {
    ($result:expr) => (stry!($result, $crate::errors::SmzdhError::InternalServerError.into_iron_error(None)));

    ($result:expr, $modifier:expr) => (match $result {
        ::std::result::Result::Ok(val) => val,
        ::std::result::Result::Err(err) => {
            info!("Error case{:?}",err);
            return ::std::result::Result::Err(
                $modifier);
        }
    })
}

#[macro_export]
macro_rules! sexpect {
    ($option:expr) => (sexpect!($option, $crate::errors::SmzdhError::ParamsError.to_response(None)));
    ($option:expr, $modifier:expr) => (match $option {
        ::std::option::Option::Some(x) => x,
        ::std::option::Option::None =>
            return ::std::result::Result::Ok(::iron::response::Response::with($modifier)),
    })
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
