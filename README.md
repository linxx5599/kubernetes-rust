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

## Rocket 基础配置 Rocket.toml

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

### 基本使用

```rust
use rocket::Route;
use serde_json::{json, Value};

#[path = "./node_service.rs"]
mod node_service;

#[get("/node")]
fn get_node() -> Value {
    let result = node_service::get_node();
    json!(result)
}

#[post("/node")]
fn create_node() -> &'static str {
    "Hello, node!"
}

// 定义一个函数来返回所有路由
pub fn routes() -> Vec<Route> {
    routes![get_node, create_node]
}

```
