use rocket::Route;
use serde_json::{json, Value};

#[path = "./namespace_service.rs"]
mod namespace_service;

#[get("/namespace")]
async fn get_namespace() -> Value {
    let result = namespace_service::get_namespace().await;
    json!(result)
}

#[post("/namespace")]
fn create_namespace() -> Value {
    json!("Hello, namespace!")
}

// 定义一个函数来返回所有路由
pub fn routes() -> Vec<Route> {
    routes![get_namespace, create_namespace]
}
