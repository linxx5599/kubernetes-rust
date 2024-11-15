extern crate rocket; // 引入Rocket宏

use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};

// 引入模块
mod config;

mod common;
use common::kube_client;
use common::utils;

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
        .attach(common::json_response::JsonResponseFairing)
        .mount("/", common::controllers::node_controller::routes())
        .mount("/", common::controllers::pod_controller::routes())
        .mount("/", common::controllers::namespace_controller::routes())
        .mount("/", common::controllers::host_controller::routes())
        .mount("/", common::controllers::vm_controller::routes())
}
