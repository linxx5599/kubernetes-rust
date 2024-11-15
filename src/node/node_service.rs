use k8s_openapi::api::core::v1::Node;
use kube::api::{DeleteParams, Patch, PatchParams, PostParams};
use kube::{api::ObjectList, Api};
use rocket::serde::json::Json;
use serde_json::{json, to_value, Value};

use crate::common_mod::get_root_error;
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

pub async fn create_node(pod_body: Json<Node>) -> Value {
    let client = kube_client::MKubeClient::new().await.unwrap();
    let nodes: Api<Node> = Api::all(client);
    let data = pod_body.into_inner();
    let params = PostParams::default();
    match nodes.create(&params, &data).await {
        Ok(pod) => json!(&pod),
        Err(err) => {
            json!({
                "code": 400,
                "message": get_root_error(&err).to_string(),
            })
        }
    }
}

pub async fn update_node(name: &str, pod_body: Json<Node>) -> Value {
    let client = kube_client::MKubeClient::new().await.unwrap();
    let nodes: Api<Node> = Api::all(client);
    let patch = pod_body.into_inner();
    let params = PatchParams::apply("myapp");
    let patch = Patch::Apply(&patch);
    match nodes.patch(name, &params, &patch).await {
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
    let nodes: Api<Node> = Api::all(client);
    let params = DeleteParams::default();
    match nodes.delete(name, &params).await {
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
