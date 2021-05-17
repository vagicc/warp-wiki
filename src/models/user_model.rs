use core::str;
use diesel::prelude::*;
use log::error;
use crate::db::PgPooledConnection;
use bcrypt::{DEFAULT_COST,verify,hash_with_salt};

#[derive(Debug, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub realname: String,
}

impl User {
    /* 身份认证 */
    pub fn authenticate(db: &PgPooledConnection, user: &str, passwd: &str) -> Option<Self> {
        use crate::schema::users::dsl::*;
        let (user, hash) = match users
            .filter(username.eq(user))
            .select(((id, username, realname), password))
            .first::<(User, String)>(db)
        {
            Ok((user, hash)) => (user, hash),
            Err(e) => {
                error!("读取用户 {:?} 信息出错: {:?}", user, e);
                return None;
            }
        };

        // let hashed=hash_with_salt(passwd, DEFAULT_COST, salt);
        /* 判断密码是否正确 */
        match verify(&passwd, &hash) {
            Ok(true) => Some(user),
            Ok(false) => None,
            Err(e) => {
                error!("密码认证失败：{:?}: {:?}", user, e);
                None
            }
        }
    }
}
