use crate::db::{pg_pool, PgPooledConnection};
use crate::models::user_model::User;
use chrono::prelude;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};
use log::{debug, error};
use rand::distributions::Alphanumeric;
use rand::thread_rng;
use rand::Rng;
use warp::{filters::BoxedFilter, Filter};

#[derive(Debug)]
pub struct NoDbReady;
impl warp::reject::Reject for NoDbReady {}

pub struct Session {
    db: PgPooledConnection,
    id: Option<i32>,
    user: Option<User>,
}

impl Session {
    /// 尝试验证该会话的用户。
    ///
    /// 如果用户名和密码有效，创建并返回一个会话密钥。
    /// 如果身份验证失败，则返回None。
    pub fn authenticate(&mut self, username: &str, passwd: &str) -> Option<String> {
        if let Some(user) = User::authenticate(&self.db, username, passwd) {
            debug!("用户({:?})密码认证", user);
            let secret = random_key(48);

            use crate::schema::sessions::dsl as s;
            let result = diesel::insert_into(s::sessions)
                .values((s::user_id.eq(user.id), s::cookie.eq(&secret)))
                .returning(s::id)
                .get_results(&self.db);
            if let Ok([a]) = result.as_ref().map(|v| &**v) {
                self.id = Some(*a);
                self.user = Some(user);
                return Some(secret);
            } else {
                error!("用户（{}）创建会话失败：{:?}", user.username, result);
            }
        }
        None
    }

    pub fn from_key(db: PgPooledConnection, sessionkey: Option<&str>) -> Self {
        use crate::schema::sessions::dsl as s;
        use crate::schema::users::dsl as u;
        let (id, user) = sessionkey
            .and_then(|sessionkey| {
                u::users
                    .inner_join(s::sessions)
                    .select((s::id, (u::id, u::username, u::realname)))
                    .filter(s::cookie.eq(&sessionkey))
                    .first::<(i32, User)>(&db)
                    .ok()
            })
            .map(|(i, u)| (Some(i), Some(u)))
            .unwrap_or((None, None));

        debug!("session: #{:?}  {:?}", id, user);
        Session { db, id, user }
    }

    /// 清除会话，但保留数据库连接池
    /// 清除session的id、user,同时删除sessions表中的数据
    pub fn clear(&mut self) {
        use crate::schema::sessions::dsl as s;
        if let Some(session_id) = self.id {
            diesel::delete(s::sessions.filter(s::id.eq(session_id)))
                .execute(&self.db)
                .map_err(|e| {
                    error!("");
                })
                .ok();
        }
        self.id = None;
        self.user = None;
    }

    pub fn user(&self) -> Option<&User> {
        self.user.as_ref()
    }
    pub fn db(&self) -> &PgPooledConnection {
        &self.db
    }
}

fn random_key(len: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .map(char::from)
        .take(len)
        .collect()
}

pub fn create_session_filter() -> BoxedFilter<(Session,)> {
    let pool = pg_pool();
    let teskk = warp::any()
        .and(warp::filters::cookie::optional("EXAUTH"))
        .and_then(move |key: Option<String>| {
            let pool = pool.clone();
            async move {
                let key = key.as_ref().map(|s| &**s);
                match pool.get() {
                    Ok(conn) => Ok(Session::from_key(conn, key)),
                    Err(e) => {
                        error!("获取数据库连接失败:{}", e);
                        Err(warp::reject::custom(NoDbReady))
                    }
                }
            }
        })
        .boxed();
    teskk
}
