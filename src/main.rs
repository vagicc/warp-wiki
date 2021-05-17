use dotenv::dotenv;
use std::{env, net::SocketAddr};

mod db;
mod filters;
mod handlers;
mod models;
mod routes;
mod schema;
mod session;

#[macro_use]
extern crate diesel;

#[tokio::main]
async fn main() {
    let routes = filters::all_routes();

    // warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    let cert_path = get_env("cert_path");
    let key_path = get_env("key_path");
    let ip_addr = get_env("ip_address");
    let socket_addr: SocketAddr = ip_addr.as_str().parse().unwrap();

    println!("站点（https://warp.wiki/）");
    println!("warp https 监听： {:?}", ip_addr);

    warp::serve(routes)
        .tls()
        .cert_path(cert_path)
        .key_path(key_path)
        .run(socket_addr)
        .await;
}

pub fn get_env(key: &str) -> String {
    dotenv().ok();
    let msg = ".env文件必须配置的环境变量：".to_string() + key;
    let value = env::var(key).expect(&msg);
    value
}
