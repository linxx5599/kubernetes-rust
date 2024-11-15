use k8s_openapi::api::core::v1::Node;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put, routes, Route};
use serde_json::{json, Value};

use crate::utils;

#[path = "./node_service.rs"]
mod node_service;

#[get("/node")]
async fn get_node() -> Value {
    let result = node_service::get_node().await;
    json!(result)
}

#[post("/node", data = "<node_body>")]
async fn create_node(node_body: Json<Node>) -> Value {
    match Some(node_body) {
        Some(params) => {
            let result = node_service::create_node(params).await;
            json!(result)
        }
        None => {
            json!({"code": 400, "message": "参数异常"})
        }
    }
}

#[put("/node?<name>", data = "<node_body>")]
async fn update_node(name: Option<&str>, node_body: Json<Node>) -> Value {
    let name = utils::validate_and_set_str::<i32>(name);
    if name.is_empty() {
        return json!({"code": 400, "message": "name is empty"});
    }
    match Some(node_body) {
        Some(params) => {
            let result = node_service::update_node(&name, params).await;
            json!(result)
        }
        None => {
            json!({"code": 400, "message": "参数异常"})
        }
    }
}

#[delete("/node?<name>")]
async fn delete_node(name: Option<&str>) -> Value {
    let name = utils::validate_and_set_str::<i32>(name);
    if name.is_empty() {
        return json!({"code": 400, "message": "name is empty"});
    }
    let result = node_service::delete_node(&name).await;
    json!(result)
}

// 定义一个函数来返回所有路由
pub fn routes() -> Vec<Route> {
    routes![get_node, create_node, update_node, delete_node]
}
