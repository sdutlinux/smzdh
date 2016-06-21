use iron::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use iron::headers::ContentType;
use iron::modifiers::Header;
use std::collections::BTreeMap;
use serde_json::value::Value as JsonValue;
use iron::status;
use iron::status::Status;
use std::convert::Into;
use serde::ser::Serialize;
use serde_json::value::to_value;
use serde_json;

pub struct Json {
    pub data:BTreeMap<String,JsonValue>,
}

impl Json {
    pub fn insert<K:Into<String>,V:Serialize+?Sized>(&mut self, k:K, v:&V) {
        self.data.insert(k.into(),to_value(v));
    }

    pub fn new() -> Self {
        Json{data:BTreeMap::new()}
    }
}

pub fn json_headers() -> Header<ContentType> {
    Header(ContentType(Mime(TopLevel::Application, SubLevel::Json,
                            vec![(Attr::Charset, Value::Utf8)])))
}


pub fn success_json_response(data:&Json) -> (Status,Header<ContentType>,String) {
    (status::Ok,Header(ContentType(Mime(TopLevel::Application, SubLevel::Json,
                                        vec![(Attr::Charset, Value::Utf8)]))),
     serde_json::to_string(&data.data).unwrap_or(String::from("{}"))
    )
}
