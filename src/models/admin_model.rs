use crate::db;
use diesel;
use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Queryable)]
pub struct Admin {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub salt: String,
    pub email: String,
    pub mobile: String,
    pub role: i32,
    pub status: i32,
}

pub fn insert_data(newdata: Admin) {
    let conn = db::establish_connection();
    println!("机型中插入数据到表");

    use crate::schema::admins::dsl::*;
    let result = diesel::insert_into(admins)
        .values((
            username.eq(newdata.username),
            password.eq(newdata.password),
            salt.eq(newdata.salt),
            email.eq(newdata.email),
            mobile.eq(newdata.mobile),
        ))
        .execute(&conn)
        .map_err(|e| format!("插入admins表出错：{}", e));
    match result {
        Ok(_) => println!("插入的admin（）成功！！"),
        Err(msg) => println!("插入admins失败：{}", msg),
    };
}
