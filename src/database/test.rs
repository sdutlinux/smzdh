use postgres::{Connection, SslMode};

/*
lazy_static! {
    static ref CONN:Connection = match Connection::connect("postgres://ipaomian:root@localhost:5432/smzdh",SslMode::None) {
        Ok(c) => Arc::new(c.) ,
        Err(e) => {
            error!("{:?}",e);
            panic!();
        },
    };
}
 */

static URL:&'static str = "postgres://ipaomian:root@localhost:5432/smzdh";

fn create_conn(url:&str) -> Connection {
    match Connection::connect(url,SslMode::None) {
        Ok(c) => c,
        Err(e) => {
            error!("Connect error:{:?}",e);
            panic!();
        },
    }
}

pub fn conn() -> Connection {
    create_conn(URL)
}

pub fn test() {
    let c = conn();
    let result = c.query("SELECT * from pg_user;", &[]);
    info!("what ? {:?}",result);
}
