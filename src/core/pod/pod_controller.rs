use crate::utils;
use k8s_openapi::api::core::v1::Pod;
use rocket::{delete, get, post, put, routes, serde::json::Json, Route};
use serde_json::{json, Value};

mod pod_service;

#[get("/pod")]
async fn get_pod() -> Value {
    let result = pod_service::get_pod().await;
    json!(result)
}

#[post("/pod", data = "<pod_body>")]
async fn create_pod(pod_body: Json<Pod>) -> Value {
    match Some(pod_body) {
        Some(params) => {
            let _params = params.clone().into_inner();
            let ns = &_params.metadata.namespace.unwrap();
            if ns.is_empty() {
                return json!({"code": 400, "message": "namespace is empty"});
            }
            let result = pod_service::create_pod(ns, params).await;
            json!(result)
        }
        None => {
            json!({"code": 400, "message": "参数异常"})
        }
    }
}

#[put("/pod?<name>&<ns>", data = "<pod_body>")]
async fn update_pod(name: Option<&str>, ns: Option<&str>, pod_body: Json<Pod>) -> Value {
    let name = utils::validate_and_set_str::<i32>(name);
    let ns = utils::validate_and_set_str::<i32>(ns);
    if name.is_empty() {
        return json!({"code": 400, "message": "name is empty"});
    }
    if ns.is_empty() {
        return json!({"code": 400, "message": "namespace is empty"});
    }
    match Some(pod_body) {
        Some(params) => {
            let result = pod_service::update_pod(&name, &ns, params).await;
            json!(result)
        }
        None => {
            json!({"code": 400, "message": "参数异常"})
        }
    }
}

#[delete("/pod?<name>&<ns>")]
async fn delete_pod(name: Option<&str>, ns: Option<&str>) -> Value {
    let name = utils::validate_and_set_str::<i32>(name);
    let ns = utils::validate_and_set_str::<i32>(ns);
    if name.is_empty() {
        return json!({"code": 400, "message": "name is empty"});
    }
    if ns.is_empty() {
        return json!({"code": 400, "message": "namespace is empty"});
    }
    let result = pod_service::delete_pod(&name, &ns).await;
    json!(result)
}

// 定义一个函数来返回所有路由
pub fn routes() -> Vec<Route> {
    routes![get_pod, create_pod, update_pod, delete_pod]
}
