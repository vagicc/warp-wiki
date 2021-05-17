use std::fmt::format;

use warp::Filter;

pub fn test() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let add = warp::get()
        .and(warp::path!("link" / "add"))
        .and(warp::path::end())
        .map(|| {
            println!("测试添加linksnap表");
            use crate::db;
            use crate::models::link_model;
            let conn = db::establish_connection();
            let newdata = link_model::AddLink {
                title: "临来笑笑生".to_string(),
                url: "https://warp.wiki".to_string(),
            };
            let link = link_model::Link::add_link(newdata, &conn);

            format!("添加")
        });
    add
}
