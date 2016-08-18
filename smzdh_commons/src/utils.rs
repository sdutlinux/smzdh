use crypto::{ symmetriccipher, buffer, aes, blockmodes };
use crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };
use rustc_serialize::hex::ToHex;

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

pub fn hex(data:&[u8]) -> String {
    data.to_hex()
}
/*
#[macro_export]
macro_rules! jget {
    ($json:expr,$key:expr,$convert:expr) => (
        $json.get($key).and_then(|tmp| {
            match $convert {
                "string" => tmp.as_string(),
                "i64" => tmp.as_i64(),
                "u64" => tmp.as_u64(),
                "f64" => tmp.as_f64(),
                "bool" => tmp.as_boolean(),
            }
        }),$crate::error::SmzdhError::ParamsError.to_response()
    )
}
*/


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
