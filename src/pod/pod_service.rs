use k8s_openapi::api::core::v1::Pod;
use kube::api::{DeleteParams, Patch, PatchParams, PostParams};
use kube::{api::ObjectList, Api};
use rocket::serde::json::Json;
use serde_json::{json, to_value, Value};

use crate::common_mod::get_root_error;
use crate::kube_client;

pub async fn get_pod() -> Value {
    let client = kube_client::MKubeClient::new().await.unwrap();
    //查询k3s下所有的pods
    let result: Result<ObjectList<Pod>, kube::Error> =
        Api::all(client).list(&Default::default()).await;
    match result {
        Ok(pods) => {
            // 处理成功的结果
            let pod_value = to_value(&pods).unwrap();
            return pod_value;
        }
        Err(err) => {
            json!({
                "code": 400,
                "message": get_root_error(&err).to_string(),
            })
        }
    }
}

pub async fn create_pod(ns: &str, pod_body: Json<Pod>) -> Value {
    let client = kube_client::MKubeClient::new().await.unwrap();
    let pods: Api<Pod> = Api::namespaced(client, ns);
    let data = pod_body.into_inner();
    let params = PostParams::default();
    match pods.create(&params, &data).await {
        Ok(pod) => json!(&pod),
        Err(err) => {
            json!({
                "code": 400,
                "message": get_root_error(&err).to_string(),
            })
        }
    }
}

pub async fn update_pod(name: &str, ns: &str, pod_body: Json<Pod>) -> Value {
    let client = kube_client::MKubeClient::new().await.unwrap();
    let pods: Api<Pod> = Api::namespaced(client, ns);
    let patch = pod_body.into_inner();
    let params = PatchParams::default();
    let patch = Patch::Apply(&patch);
    match pods.patch(name, &params, &patch).await {
        Ok(pod) => json!(&pod),
        Err(err) => {
            json!({
                "code": 400,
                "message": get_root_error(&err).to_string(),
            })
        }
    }
}

pub async fn delete_pod(name: &str, ns: &str) -> Value {
    let client = kube_client::MKubeClient::new().await.unwrap();
    let pods: Api<Pod> = Api::namespaced(client, ns);
    let params = DeleteParams::default();
    match pods.delete(name, &params).await {
        Ok(resp) => match &resp.left() {
            Some(pod) => json!(pod),
            None => json!({
                "code": 400,
                "message": format!("pods {} not found: NotFound", name),
            }),
        },
        Err(err) => {
            json!({
                "code": 400,
                "message": get_root_error(&err).to_string(),
            })
        }
    }
}
