use crypto::digest::Digest;
use crypto::sha2::Sha512;
use rustc_serialize::hex::ToHex;
use rand::{ Rng, OsRng };
use iron::Url;

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
