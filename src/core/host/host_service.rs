use crate::common::api::host::Host;
use crate::utils::get_root_error;
use crate::{kube_client, utils};
use kube::api::{DeleteParams, ListParams, PostParams};
use kube::core::response::StatusSummary;
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
    let host_api: Api<Host> = Api::<Host>::all(client);
    let result: Result<ObjectList<Host>, kube::Error> = host_api.list(&list_params).await;
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

pub async fn create_host(host_body: Json<Host>) -> Value {
    let client = kube_client::MKubeClient::new().await.unwrap();
    let host_api: Api<Host> = Api::all(client);
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

pub async fn update_host(name: &str, host_body: Json<Host>) -> Value {
    let client = kube_client::MKubeClient::new().await.unwrap();
    let host_api: Api<Host> = Api::all(client);
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
    let host_api: Api<Host> = Api::all(client);
    let params = DeleteParams::default();
    match host_api.delete(name, &params).await {
        Ok(resp) => match &resp.right() {
            Some(status) => {
                if &status.status.unwrap() == &StatusSummary::Success {
                    return json!(&status.details);
                }
                return json!({
                    "code": 400,
                    "message": &status.message,
                });
            }
            None => json!({
                "code": 400,
                "message": format!("hosts.virt.cum.io hosts {} not found: NotFound", name),
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
