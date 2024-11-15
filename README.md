## Rust编译器和工具链

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

```

```bash
rustup update stable
```

## start

```bash
cargo run
```

## build

```bash
cargo build --release
```

## check

```bash
cargo check
```

## 文件目录

```bash
code
 ├─src
 │  ├─common               # 公共模块
 │  │  ├─api               # crd资源的定义
 │  │  ├─controllers       # 统一路由控制器
 │  │  ├─json_response     # 封装的返回结果
 │  │  ├─kube_client       # k8s client
 │  │  ├─mod               # ..
 │  │  └─utils             # 工具类
 │  ├─config               # 配置文件
 │  │  ├─crd               # crd资源名称、资源组、版本等配置
 │  │  └─mod               # ..
 │  ├─core                 # 源码的实现
 │  │  ├─host
 │  │  ├─namespace
 │  │  ├─node
 │  │  └─pod
 │  └─main.rs              # 入口文件
 └─Rocket.toml             # rocket配置文件
```

## Rocket 配置文件 Rocket.toml

```bash
# Rocket默认配置
[default]
limits = { form = "64 kB", json = "1 MiB" }

[debug]
address = "127.0.0.1"  # 服务器监听的IP地址
port = 8000  # 服务器监听的端口号
# only the `json` key from `default` will be overridden; `form` will remain
limits = { json = "10MiB" }

# 生产环境配置
[release]
address = "0.0.0.0"  # 服务器监听的IP地址
ip_header = false
port = 8000  # 使用8000端口以处理HTTP请求
```

## 基本使用 已 node 资源为例子

### node_controller.rs 内容

```rust
use crate::utils;
use k8s_openapi::api::core::v1::Node;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put, routes, Route};
use serde_json::{json, Value};

mod node_service;

#[get("/node")]
async fn get_node() -> Value {
    let result = node_service::get_node().await;
    json!(result)
}

#[post("/node", data = "<node_body>")]
async fn create_node(node_body: Json<Node>) -> Value {
    match Some(node_body) {
        Some(params) => {
            let result = node_service::create_node(params).await;
            json!(result)
        }
        None => {
            json!({"code": 400, "message": "参数异常"})
        }
    }
}

#[put("/node?<name>", data = "<node_body>")]
async fn update_node(name: Option<&str>, node_body: Json<Node>) -> Value {
    let name = utils::validate_and_set_str::<i32>(name);
    if name.is_empty() {
        return json!({"code": 400, "message": "name is empty"});
    }
    match Some(node_body) {
        Some(params) => {
            let result = node_service::update_node(&name, params).await;
            json!(result)
        }
        None => {
            json!({"code": 400, "message": "参数异常"})
        }
    }
}

#[delete("/node?<name>")]
async fn delete_node(name: Option<&str>) -> Value {
    let name = utils::validate_and_set_str::<i32>(name);
    if name.is_empty() {
        return json!({"code": 400, "message": "name is empty"});
    }
    let result = node_service::delete_node(&name).await;
    json!(result)
}

// 定义一个函数来返回所有路由
pub fn routes() -> Vec<Route> {
    routes![get_node, create_node, update_node, delete_node]
}
```

### node_service.rs 内容

```rust
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
```

### common/controllers.rs 导入

```bash
#[path = "../core/node/node_controller.rs"]
pub mod node_controller;
```

### main.rs 注册

```rust
extern crate rocket; // 引入Rocket宏

#[rocket::launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build().mount("/", common::controllers::node_controller::routes())
}

```
