use k8s_openapi::api::core::v1::Node;
use kube::api::{DeleteParams, PostParams};
use kube::{api::ObjectList, Api};
use rocket::serde::json::Json;
use serde_json::{json, to_value, Value};

use crate::utils::get_root_error;
use crate::kube_client;

pub async fn get_node() -> Value {
    let client = kube_client::MKubeClient::new().await.unwrap();
    //查询k3s下所有的nodes
    let result: Result<ObjectList<Node>, kube::Error> =
        Api::all(client).list(&Default::default()).await;
    match result {
        Ok(nodes) => {
            // 处理成功的结果
            let node_value = to_value(&nodes).unwrap();
            return node_value;
        }
        Err(err) => {
            json!({
                "code": 400,
                "message": get_root_error(&err).to_string(),
            })
        }
    }
}

pub async fn create_node(node_body: Json<Node>) -> Value {
    let client = kube_client::MKubeClient::new().await.unwrap();
    let node_api: Api<Node> = Api::all(client);
    let data = node_body.into_inner();
    let params = PostParams::default();
    match node_api.create(&params, &data).await {
        Ok(pod) => json!(&pod),
        Err(err) => {
            json!({
                "code": 400,
                "message": get_root_error(&err).to_string(),
            })
        }
    }
}

pub async fn update_node(name: &str, node_body: Json<Node>) -> Value {
    let client = kube_client::MKubeClient::new().await.unwrap();
    let node_api: Api<Node> = Api::all(client);
    let mut data = node_body.into_inner();
    //判断是否有 data.metadata.resourceVersion
    if data.metadata.resource_version.is_none() {
        match node_api.get(name).await {
            Ok(host) => {
                data.metadata.resource_version = host.metadata.resource_version;
            }
            Err(err) => {
                return json!({
                    "code": 400,
                    "message": get_root_error(&err).to_string(),
                });
            }
        }
    }
    let params = PostParams::default();
    match node_api.replace(name, &params, &data).await {
        Ok(pod) => json!(&pod),
        Err(err) => {
            json!({
                "code": 400,
                "message": get_root_error(&err).to_string(),
            })
        }
    }
}

pub async fn delete_node(name: &str) -> Value {
    let client = kube_client::MKubeClient::new().await.unwrap();
    let node_api: Api<Node> = Api::all(client);
    let params = DeleteParams::default();
    match node_api.delete(name, &params).await {
        Ok(resp) => match &resp.left() {
            Some(pod) => json!(pod),
            None => json!({
                "code": 400,
                "message": format!("nodes {} not found: NotFound", name),
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
