macro_rules! check_sl {
    ($skip:ident,$limit:ident,$url:expr) => (
        let ($skip,$limit) = ::smzdh_commons::utils::skip_limit($url);
        check!(
            (0 < $skip) && (0 < $limit && $limit <= 1000),
            SError::ParamsError,"skip 的范围为 0-100，limit 的范围为 0-1000"
        );
    )
}

pub mod users;
pub mod posts;
pub mod comments;
pub mod category;
