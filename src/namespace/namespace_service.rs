use crate::common_mod::get_root_error;
use crate::kube_client;
use k8s_openapi::api::core::v1::Namespace;
use kube::api::{DeleteParams, Patch, PatchParams, PostParams};
use kube::{api::ObjectList, Api};
use rocket::serde::json::Json;
use serde_json::{json, to_value, Value};

pub async fn get_namespace() -> Value {
    let client = kube_client::MKubeClient::new().await.unwrap();
    //查询k3s下所有的namespaces
    let result: Result<ObjectList<Namespace>, kube::Error> =
        Api::all(client).list(&Default::default()).await;
    match result {
        Ok(namespaces) => {
            // 处理成功的结果
            let namespace_value = to_value(&namespaces).unwrap();
            return namespace_value;
        }
        Err(err) => {
            json!({
                "code": 400,
                "message": get_root_error(&err).to_string(),
            })
        }
    }
}

pub async fn create_namespace(pod_body: Json<Namespace>) -> Value {
    let client = kube_client::MKubeClient::new().await.unwrap();
    let namespaces: Api<Namespace> = Api::all(client);
    let data = pod_body.into_inner();
    let params = PostParams::default();
    match namespaces.create(&params, &data).await {
        Ok(pod) => json!(&pod),
        Err(err) => {
            json!({
                "code": 400,
                "message": get_root_error(&err).to_string(),
            })
        }
    }
}

pub async fn update_namespace(name: &str, pod_body: Json<Namespace>) -> Value {
    let client = kube_client::MKubeClient::new().await.unwrap();
    let namespaces: Api<Namespace> = Api::all(client);
    let patch = pod_body.into_inner();
    let params = PatchParams::apply("myapp");
    let patch = Patch::Apply(&patch);
    match namespaces.patch(name, &params, &patch).await {
        Ok(pod) => json!(&pod),
        Err(err) => {
            json!({
                "code": 400,
                "message": get_root_error(&err).to_string(),
            })
        }
    }
}

pub async fn delete_namespace(name: &str) -> Value {
    let client = kube_client::MKubeClient::new().await.unwrap();
    let namespaces: Api<Namespace> = Api::all(client);
    let params = DeleteParams::default();
    match namespaces.delete(name, &params).await {
        Ok(resp) => match &resp.left() {
            Some(pod) => json!(pod),
            None => json!({
                "code": 400,
                "message": format!("namespaces {} not found: NotFound", name),
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
