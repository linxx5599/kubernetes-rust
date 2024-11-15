use crate::common_mod::get_root_error;
use crate::{host, kube_client, utils};
use kube::api::{DeleteParams, ListParams, PostParams};
use kube::{api::ObjectList, Api};
use rocket::serde::json::Json;
use serde_json::{json, to_value, Value};

pub async fn get_host(params: utils::PaginationParams) -> Value {
    let client = kube_client::MKubeClient::new().await.unwrap();
    //查询k3s下所有的hosts
    let mut list_params = ListParams::default();
    if params.limit != "" {
        list_params.limit = params.limit.parse::<u32>().ok();
    }
    let host_api: Api<host::Host> = Api::<host::Host>::all(client);
    let result: Result<ObjectList<host::Host>, kube::Error> = host_api.list(&list_params).await;
    match result {
        Ok(hosts) => {
            // 处理成功的结果
            let host_value = to_value(&hosts).unwrap();
            return host_value;
        }
        Err(err) => {
            json!({
                "code": 400,
                "message": get_root_error(&err).to_string(),
            })
        }
    }
}

pub async fn create_host(host_body: Json<host::Host>) -> Value {
    let client = kube_client::MKubeClient::new().await.unwrap();
    let host_api: Api<host::Host> = Api::all(client);
    let data = host_body.into_inner();
    let params = PostParams::default();
    match host_api.create(&params, &data).await {
        Ok(host) => json!(&host),
        Err(err) => {
            json!({
                "code": 400,
                "message": get_root_error(&err).to_string(),
            })
        }
    }
}

pub async fn update_host(name: &str, host_body: Json<host::Host>) -> Value {
    let client = kube_client::MKubeClient::new().await.unwrap();
    let host_api: Api<host::Host> = Api::all(client);
    let mut data = host_body.into_inner();
    //判断是否有 data.metadata.resourceVersion
    if data.metadata.resource_version.is_none() {
        match host_api.get(name).await {
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
    match host_api.replace(name, &params, &data).await {
        Ok(host) => json!(&host),
        Err(err) => {
            json!({
                "code": 400,
                "message": get_root_error(&err).to_string(),
            })
        }
    }
}

pub async fn delete_host(name: &str) -> Value {
    let client = kube_client::MKubeClient::new().await.unwrap();
    let host_api: Api<host::Host> = Api::all(client);
    let params = DeleteParams::default();
    match host_api.delete(name, &params).await {
        Ok(resp) => match &resp.left() {
            Some(host) => json!(host),
            None => json!({
                "code": 400,
                "message": format!("hosts {} not found: NotFound", name),
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
