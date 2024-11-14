extern crate rocket; // 引入Rocket宏

use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};

// 引入模块
#[path = "./lib/lib.rs"]
mod lib;
use lib::common_mod;
use lib::controllers;
use lib::kube_client;
use lib::utils;

// 启动Rocket服务器并挂载路由
#[rocket::launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    // 设置CORS配置 attach(cors)
    let allowed_origins = AllowedOrigins::all();
    let cors = CorsOptions {
        allowed_origins,
        allowed_headers: AllowedHeaders::all(),
        ..Default::default()
    }
    .to_cors()
    .expect("Failed to create CORS");
    rocket::build()
        .attach(cors)
        .attach(common_mod::JsonResponseFairing)
        .mount("/", controllers::node_controller::routes())
        .mount("/", controllers::pod_controller::routes())
        .mount("/", controllers::host_controller::routes())
        .mount("/", controllers::namespace_controller::routes())
}
