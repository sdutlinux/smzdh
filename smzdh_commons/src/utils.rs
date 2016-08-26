use crypto::digest::Digest;
use crypto::sha2::Sha512;
use rustc_serialize::hex::ToHex;
use rustc_serialize::base64::{STANDARD,ToBase64,FromBase64};
use rand::{ Rng, OsRng };
use iron::Url;

pub fn encrypt(pass:&str) -> (String,String) {

    let mut rng = OsRng::new().ok().unwrap();
    let mut salt = [0;32];
    rng.fill_bytes(&mut salt);
    let mut hasher = Sha512::new();
    let e = [&salt,pass.as_bytes()].concat();
    hasher.input(&*e);
    let mut vec = [0;64];
    hasher.result(&mut vec);
    (vec.to_base64(STANDARD),salt.to_base64(STANDARD))
}

pub fn hex(data:&[u8]) -> String {
    data.to_hex()
}

pub fn check_pass(p:&str,ep:&str,salt:&str) -> bool {
    let mut hasher = Sha512::new();
    let bsalt = match salt.from_base64() {
        Ok(x) => x,
        Err(e) => {
            info!("salt paser to [u8] fail {:?}",e);
            return false;
        }
    };
    hasher.input(&*[&*bsalt, p.as_bytes()].concat());
    let mut vec = [0;64];
    hasher.result(&mut vec);
    vec.to_base64(STANDARD) == ep
}

pub fn get_query_params<'a>(params:&'a Url,key:&str) -> Option<&'a str> {
    let query_params = match params.query() {
        Some(x) => x,
        None => return None,
    };
    let p = query_params.split('&').map(|x| {
        let mut tmp = x.split('=');
        (tmp.next(),tmp.next())
    }).filter(|x| {
        if x.0.is_some() && x.1.is_some() {
            x.0.unwrap() == key
        } else { false }
    }).collect::<Vec<(Option<&str>, Option<&str>)>>();
    if p.len() == 1 {
        p[0].1
    } else {
        None
    }
}
