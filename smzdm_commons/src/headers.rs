use iron::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use iron::headers::ContentType;
use iron::modifiers::Header;

pub fn json_headers() -> Header<ContentType> {
    Header(ContentType(Mime(TopLevel::Application, SubLevel::Json,
                            vec![(Attr::Charset, Value::Utf8)])))
}
