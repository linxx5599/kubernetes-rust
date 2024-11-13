use crate::utils;
use rocket::Route;
use serde_json::{json, Value};

#[path = "./host_service.rs"]
mod host_service;

#[get("/host?<limit>")]
async fn get_host(limit: Option<&str>) -> Value {
    let limit = utils::validate_and_set_value::<i32>(limit, "10");
    let params = utils::PaginationParams { limit };
    //接收query参数
    let result = host_service::get_host(params).await;
    json!(result)
}

#[post("/host")]
fn create_host() -> Value {
    json!("Hello, host!")
}

// 定义一个函数来返回所有路由
pub fn routes() -> Vec<Route> {
    routes![get_host, create_host]
}
