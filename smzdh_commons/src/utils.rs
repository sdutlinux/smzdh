use crypto::digest::Digest;
use crypto::sha2::Sha512;
use crypto::{ symmetriccipher, buffer, aes, blockmodes };
use crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };
use rustc_serialize::hex::ToHex;
use rustc_serialize::base64::{STANDARD,ToBase64,FromBase64};
use rand::{ Rng, OsRng };
use iron::Url;

pub fn sha_encrypt(pass:&str) -> (String,String) {

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

pub fn encrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut encryptor = aes::cbc_encryptor(
        aes::KeySize::KeySize256,
        key,
        iv,
        blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = try!(encryptor.encrypt(&mut read_buffer, &mut write_buffer, true));
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().cloned());
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }
    Ok(final_result)
}

pub fn decrypt(encrypted_data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut decryptor = aes::cbc_decryptor(
        aes::KeySize::KeySize256,
        key,
        iv,
        blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = try!(decryptor.decrypt(&mut read_buffer, &mut write_buffer, true));
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().cloned());
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }
    Ok(final_result)
}

pub fn hello() {
    let message = "Hello World!";
    // In a real program, the key and iv may be determined
    // using some other mechanism. If a password is to be used
    // as a key, an algorithm like PBKDF2, Bcrypt, or Scrypt (all
    // supported by Rust-Crypto!) would be a good choice to derive
    // a password. For the purposes of this example, the key and
    // iv are just random values.
    //let key = [0;32];
    //let iv = [0;16];
    let encrypted_data = encrypt(message.as_bytes(), KEY, SECRET).ok().unwrap();
    let decrypted_data = decrypt(&encrypted_data[..], KEY, SECRET).ok().unwrap();

    info!("{:?}",decrypted_data);
    info!("{:?}",message.as_bytes().to_vec());
}

static SECRET:&'static [u8] = b"smzdhaxibaahouga";
static KEY:&'static [u8] = b"qmfygnlgabhuxtgcpwkxdzxquhhbbqxw";

pub fn encrypt_cookie(data:&[u8],salt:&str) ->  Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let bsalt = match salt.from_base64() {
        Ok(s) => s,
        Err(e) => {
            info!("from base64 fail {:?}",e);
            unreachable!();
        },
    };
    encrypt(&[data,&bsalt].concat(),KEY,SECRET)
}

pub fn decrypt_cookie(edata:&[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    decrypt(edata,KEY,SECRET)
}
