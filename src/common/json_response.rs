use std::io::Cursor;

use regex::Regex;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Status;
use rocket::tokio::io::AsyncReadExt;
use rocket::Request;
use rocket::Response;
use serde_json::json;

pub struct JsonResponseFairing;

#[rocket::async_trait]
impl Fairing for JsonResponseFairing {
    fn info(&self) -> Info {
        Info {
            name: "JSON Response Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) -> () {
        match response.body_mut() {
            body => {
                let mut body_bytes: Vec<u8> = Vec::new();
                match body.into() {
                    Some(data) => {
                        data.read_to_end(&mut body_bytes).await.unwrap();
                        let body_str = String::from_utf8(body_bytes).unwrap();
                        let mut _response =
                            json!({"status": &response.status().code ,"message": body_str });
                        let mut status_code = _response["status"].as_u64().unwrap() as u16;
                        // 判断code小于300，则给_response添加data字段
                        if status_code < 300 {
                            let body_json: serde_json::Value =
                                serde_json::from_str(&body_str).unwrap();
                            // 如果body_json是object
                            if body_json.is_object()
                                && body_json.as_object().unwrap().contains_key("code")
                            {
                                let body_json = body_json.as_object().unwrap();
                                status_code = body_json["code"].as_u64().unwrap() as u16;
                                _response["status"] = status_code.clone().into();
                                if body_json.contains_key("message") {
                                    _response["message"] = body_json["message"].clone();
                                }
                            } else {
                                _response["data"] = body_json.clone().into();
                                _response["message"] = "操作成功".into();
                            }
                        } else {
                            let re = Regex::new(r#"<title>(.*?)</title>"#).unwrap();
                            if let Some(captures) = re.captures(&body_str) {
                                if let Some(title_content) = captures.get(1) {
                                    _response["message"] = title_content.as_str().into();
                                }
                            }
                        }
                        response.set_status(Status::from_code(status_code).unwrap());
                        //给_response添加data字段
                        let cursor = Cursor::new(_response.to_string());
                        response.set_sized_body(cursor.get_ref().len(), cursor);
                    }
                    None => {
                        let json_body = json!({ "status": Status::InternalServerError.code, "message": "Internal Server Error" });
                        response.set_status(
                            Status::from_code(Status::InternalServerError.code).unwrap(),
                        );
                        let cursor = Cursor::new(json_body.to_string());
                        response.set_sized_body(cursor.get_ref().len(), cursor);
                    }
                }
            }
        }
    }
}
