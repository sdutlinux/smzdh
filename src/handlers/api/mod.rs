macro_rules! check_sl {
    ($skip:ident,$limit:ident,$url:expr) => (
        let ($skip,$limit) = ::smzdh_commons::utils::skip_limit($url);
        check!(
            (0 <= $skip) && (0 < $limit && $limit <= 1000),
            SError::ParamsError,"skip 的范围为 0-100，limit 的范围为 0-1000"
        );
    )
}

macro_rules! self_user {
    ($id:ident,$str_id:expr,$uid:expr) => (
        let $id:i32;
        if $str_id == "self" {
            $id = $uid
        } else {
            $id = stry!(
                $str_id.parse::<i32>()
                    .map_err(|_| SError::ParamsError),
                "user_id 格式错误。"
            )
        }
    )
}

macro_rules! json {
    ($json:ident,$req:expr) => (
        let $json = stry!($req.extensions.get::<Json>().ok_or(SError::ParamsError)
                          ,"Json 格式错误");
    )
}

macro_rules! uid {
    ($v:ident,$req:expr) => {
        let $v = stry!($req.extensions.get::<Cookies>().ok_or(SError::UserNotLogin));
    }
}

macro_rules! user {
    ($uid:expr,$v:ident,$redis:expr,$postgres:expr) => {
        let $v: ::smzdh_commons::databases::User = stry!(
            try_caching!(
                $redis,format!("user_{}",$uid),
                databases::find_user_by_id($postgres,$uid)
            )
        );
    }
}

pub mod users;
pub mod posts;
pub mod comments;
pub mod category;
