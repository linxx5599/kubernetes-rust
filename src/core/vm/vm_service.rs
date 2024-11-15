use crate::common::api::vm::Vm;
use crate::utils::get_root_error;
use crate::{kube_client, utils};
use kube::api::{DeleteParams, ListParams, PostParams};
use kube::core::response::StatusSummary;
use kube::{api::ObjectList, Api};
use rocket::serde::json::Json;
use serde_json::{json, to_value, Value};

pub async fn get_vm(params: utils::PaginationParams) -> Value {
    let client = kube_client::MKubeClient::new().await.unwrap();
    //查询k3s下所有的vms
    let mut list_params = ListParams::default();
    if params.limit != "" {
        list_params.limit = params.limit.parse::<u32>().ok();
    }
    let vm_api: Api<Vm> = Api::<Vm>::all(client);
    let result: Result<ObjectList<Vm>, kube::Error> = vm_api.list(&list_params).await;
    match result {
        Ok(vms) => {
            // 处理成功的结果
            let vm_value = to_value(&vms).unwrap();
            return vm_value;
        }
        Err(err) => {
            json!({
                "code": 400,
                "message": get_root_error(&err).to_string(),
            })
        }
    }
}

pub async fn create_vm(vm_body: Json<Vm>) -> Value {
    let client = kube_client::MKubeClient::new().await.unwrap();
    let vm_api: Api<Vm> = Api::all(client);
    let data = vm_body.into_inner();
    let params = PostParams::default();
    match vm_api.create(&params, &data).await {
        Ok(vm) => json!(&vm),
        Err(err) => {
            json!({
                "code": 400,
                "message": get_root_error(&err).to_string(),
            })
        }
    }
}

pub async fn update_vm(name: &str, vm_body: Json<Vm>) -> Value {
    let client = kube_client::MKubeClient::new().await.unwrap();
    let vm_api: Api<Vm> = Api::all(client);
    let mut data = vm_body.into_inner();
    //判断是否有 data.metadata.resourceVersion
    if data.metadata.resource_version.is_none() {
        match vm_api.get(name).await {
            Ok(vm) => {
                data.metadata.resource_version = vm.metadata.resource_version;
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
    match vm_api.replace(name, &params, &data).await {
        Ok(vm) => json!(&vm),
        Err(err) => {
            json!({
                "code": 400,
                "message": get_root_error(&err).to_string(),
            })
        }
    }
}

pub async fn delete_vm(name: &str) -> Value {
    let client = kube_client::MKubeClient::new().await.unwrap();
    let vm_api: Api<Vm> = Api::all(client);
    let params = DeleteParams::default();
    match vm_api.delete(name, &params).await {
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
                "message": format!("vms.virt.cum.io vms {} not found: NotFound", name),
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
