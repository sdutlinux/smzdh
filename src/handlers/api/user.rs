use iron::prelude::*;
use iron::status;
/*
fn sign_in(req: &mut Request) -> IronResult(Response) {
req.extensions.insert::<ResponseTime>(precise_time_ns());
}
*/

pub fn test(req: &mut Request) -> IronResult<Response> {
    info!("Some thing {:?}",req.extensions.len());
    info!("{:?} \n {:?}",req,req.headers);
    //println!("{}",serde_json::to_string(&BTreeMap::<String,String>::new()).unwrap());
    Ok(Response::with((status::Ok, "Pong")))
}
