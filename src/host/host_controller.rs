use crate::common::api::host::Host;
use crate::utils;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put, routes, Route};
use serde_json::{json, Value};
#[path = "./host_service.rs"]
mod host_service;

#[get("/host?<limit>")]
async fn get_host(limit: Option<&str>) -> Value {
    let limit = utils::validate_and_set_value::<i32>(limit);
    let params = utils::PaginationParams { limit };
    //接收query参数
    let result = host_service::get_host(params).await;
    json!(result)
}

#[post("/host", data = "<host_body>")]
async fn create_host(host_body: Json<Host>) -> Value {
    match Some(host_body) {
        Some(params) => {
            let result = host_service::create_host(params).await;
            json!(result)
        }
        None => {
            json!({"code": 400, "message": "参数异常"})
        }
    }
}

#[put("/host?<name>", data = "<host_body>")]
async fn update_host(name: Option<&str>, host_body: Json<Host>) -> Value {
    let name = utils::validate_and_set_str::<i32>(name);
    if name.is_empty() {
        return json!({"code": 400, "message": "name is empty"});
    }
    match Some(host_body) {
        Some(params) => {
            let result = host_service::update_host(&name, params).await;
            json!(result)
        }
        None => {
            json!({"code": 400, "message": "参数异常"})
        }
    }
}

#[delete("/host?<name>")]
async fn delete_host(name: Option<&str>) -> Value {
    let name = utils::validate_and_set_str::<i32>(name);
    if name.is_empty() {
        return json!({"code": 400, "message": "name is empty"});
    }
    let result = host_service::delete_host(&name).await;
    json!(result)
}

// 定义一个函数来返回所有路由
pub fn routes() -> Vec<Route> {
    routes![get_host, create_host, update_host, delete_host]
}
