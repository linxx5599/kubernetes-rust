#[macro_use]
extern crate rocket; // 引入Rocket宏

#[path = "./controllers.rs"]
mod controllers;

use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};

#[path = "./utils.rs"]
mod utils;

#[path = "./mod.rs"]
mod common_mod;
#[path = "./client.rs"]
mod kube_client;

// 启动Rocket服务器并挂载路由
#[launch]
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
