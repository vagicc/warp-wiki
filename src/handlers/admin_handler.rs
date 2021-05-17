use crate::models::admin_model;
use warp::{Rejection, Reply};

pub async fn add_test() -> std::result::Result<impl Reply, Rejection> {
    println!("这里开测试添加admin表数据");

    let newadmin = admin_model::Admin {
        id: 2,
        username: "临来12".to_string(),
        password: "kkk".to_string(),
        salt: "ksad".to_string(),
        email: "kasdf@ek.com".to_string(),
        mobile: "132232".to_string(),
        role: 1,
        status: 1,
    };

    admin_model::insert_data(newadmin);

    let html = "添加一条admin".to_string();
    Ok(warp::reply::html(html))
}
