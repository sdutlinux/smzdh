use hyper::client::Client;
use hyper::header::{Headers,ContentType,Authorization};
use hyper::mime::{Mime, TopLevel, SubLevel,Attr, Value};
use hyper::header::Basic;

use rand::{self,Rng};

pub fn send_email(context:&str,to:&[&str]) {
    let c = Client::new();
    let boundary = rand::thread_rng()
        .gen_ascii_chars()
        .take(20)
        .collect::<String>();
    let mut header = Headers::new();
    header.set(ContentType(Mime(
        TopLevel::Multipart,SubLevel::FormData,
        vec![(Attr::Ext(String::from("boundary")),
              Value::Ext(boundary.clone()))])));
    header.set(Authorization(
        Basic{
            username:String::from("api"),
            password:Some(String::from("key-9g8x4gzcxcid-37xnhw64qgzzpu16q34")),
        }
    ));
    let mut body = Vec::new();
    body.push(("from","Smzdh<postmaster@sandbox85843.mailgun.org>"));
    for x in to {
        body.push(("to",x));
    }
    body.push(("subject","hello"));
    body.push(("html",context));

    let bs = body.iter().fold(
        String::new(),
        |acc,x| {
            format!("{}--{}\r\nContent-Disposition: form-data; name=\"{}\"\r\n\r\n{}\r\n",&acc[..],&boundary[..],x.0,x.1)
        });
    let body_str = format!("{}--{}--",&bs[..],&boundary[..]);
    let result = c.post("https://api.mailgun.net/v3/sandbox85843.mailgun.org/messages")
        .headers(header)
        .body(&body_str[..])
        .send();
    match result {
        Ok(_) => {
            info!("Send email:{} to {} success.",context,to.join(","));
        },
        Err(e) => {
            info!("Send email:{} to {} fail:{:?}.",context,to.join(","),e);
        }
    };
}
