use k8s_openapi::api::core::v1::Namespace;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put, routes, Route};
use serde_json::{json, Value};

use crate::lib::utils;

#[path = "./namespace_service.rs"]
mod namespace_service;

#[get("/namespace")]
async fn get_namespace() -> Value {
    let result = namespace_service::get_namespace().await;
    json!(result)
}

#[post("/namespace", data = "<namespace_body>")]
async fn create_namespace(namespace_body: Json<Namespace>) -> Value {
    match Some(namespace_body) {
        Some(params) => {
            let result = namespace_service::create_namespace(params).await;
            json!(result)
        }
        None => {
            json!({"code": 400, "message": "参数异常"})
        }
    }
}

#[put("/namespace?<name>", data = "<namespace_body>")]
async fn update_namespace(name: Option<&str>, namespace_body: Json<Namespace>) -> Value {
    let name = utils::validate_and_set_str::<i32>(name);
    if name.is_empty() {
        return json!({"code": 400, "message": "name is empty"});
    }
    match Some(namespace_body) {
        Some(params) => {
            let result = namespace_service::update_namespace(&name, params).await;
            json!(result)
        }
        None => {
            json!({"code": 400, "message": "参数异常"})
        }
    }
}

#[delete("/namespace?<name>")]
async fn delete_namespace(name: Option<&str>) -> Value {
    let name = utils::validate_and_set_str::<i32>(name);
    if name.is_empty() {
        return json!({"code": 400, "message": "name is empty"});
    }
    let result = namespace_service::delete_namespace(&name).await;
    json!(result)
}

// 定义一个函数来返回所有路由
pub fn routes() -> Vec<Route> {
    routes![get_namespace, create_namespace, update_namespace, delete_namespace]
}
