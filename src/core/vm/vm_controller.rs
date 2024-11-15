use crate::common::api::vm::Vm;
use crate::utils;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put, routes, Route};
use serde_json::{json, Value};

mod vm_service;

#[get("/vm?<limit>")]
async fn get_vm(limit: Option<&str>) -> Value {
    let limit = utils::validate_and_set_value::<i32>(limit);
    let params = utils::PaginationParams { limit };
    //接收query参数
    let result = vm_service::get_vm(params).await;
    json!(result)
}

#[post("/vm", data = "<vm_body>")]
async fn create_vm(vm_body: Json<Vm>) -> Value {
    match Some(vm_body) {
        Some(params) => {
            let result = vm_service::create_vm(params).await;
            json!(result)
        }
        None => {
            json!({"code": 400, "message": "参数异常"})
        }
    }
}

#[put("/vm?<name>", data = "<vm_body>")]
async fn update_vm(name: Option<&str>, vm_body: Json<Vm>) -> Value {
    let name = utils::validate_and_set_str::<i32>(name);
    if name.is_empty() {
        return json!({"code": 400, "message": "name is empty"});
    }
    match Some(vm_body) {
        Some(params) => {
            let result = vm_service::update_vm(&name, params).await;
            json!(result)
        }
        None => {
            json!({"code": 400, "message": "参数异常"})
        }
    }
}

#[delete("/vm?<name>")]
async fn delete_vm(name: Option<&str>) -> Value {
    let name = utils::validate_and_set_str::<i32>(name);
    if name.is_empty() {
        return json!({"code": 400, "message": "name is empty"});
    }
    let result = vm_service::delete_vm(&name).await;
    json!(result)
}

// 定义一个函数来返回所有路由
pub fn routes() -> Vec<Route> {
    routes![get_vm, create_vm, update_vm, delete_vm]
}
