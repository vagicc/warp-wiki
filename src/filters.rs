use crate::routes::admin_route;
use crate::routes::link_route;
use warp::Filter;

pub fn all_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let home = warp::get().and(warp::path::end()).map(|| {
        println!("访问：GET /");
        format!("首页")
    });

    let admin = admin_route::add_admin();

    let link = link_route::test();

    let routes = home.or(admin).or(link);
    routes
}
