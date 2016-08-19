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
