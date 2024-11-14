use kube::api::{ListParams, ObjectList};
use kube::Api;
use serde_json::{json, to_value, Value};

use crate::common_mod::get_root_error;
use crate::{kube_client, utils};

#[path = "./host.rs"]
mod host;

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
                "code": 500,
                "message": get_root_error(&err).to_string(),
            })
        }
    }
}
