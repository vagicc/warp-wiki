use crate::handlers::admin_handler;
use warp::Filter;

pub fn add_admin() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let add = warp::get()
        .and(warp::path!("admin" / "add"))
        .and(warp::path::end())
        .and_then(admin_handler::add_test);
    add
}
