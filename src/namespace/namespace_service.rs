use k8s_openapi::api::core::v1::Namespace;
use kube::{api::ObjectList, Api};
use serde_json::{json, to_value, Value};

use crate::common_mod::get_root_error;
use crate::kube_client;

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
            let mut msg = String::from("504: Gateway Timeout");
            msg.push_str(&get_root_error(&err).to_string());
            json!(&msg)
        }
    }
}
