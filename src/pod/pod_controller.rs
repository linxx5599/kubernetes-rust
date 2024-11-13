use rocket::Route;
use serde_json::{json, Value};

#[path = "./pod_service.rs"]
mod pod_service;

#[get("/pod")]
async fn get_pod() -> Value {
    let result = pod_service::get_pod().await;
    json!(result)
}

#[post("/pod")]
fn create_pod() -> Value {
    json!("Hello, pod!")
}

// 定义一个函数来返回所有路由
pub fn routes() -> Vec<Route> {
    routes![get_pod, create_pod]
}
