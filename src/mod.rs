use std::io::Cursor;

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
                            if body_json.is_object() {
                                let body_json = body_json.as_object().unwrap();
                                if body_json.contains_key("code") {
                                    status_code = body_json["code"].as_u64().unwrap() as u16;
                                    _response["status"] = status_code.clone().into();
                                    if body_json.contains_key("message") {
                                        _response["message"] = body_json["message"].clone();
                                    }
                                }
                            } else {
                                _response["data"] = body_json;
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

pub fn get_root_error(err: &dyn std::error::Error) -> &dyn std::error::Error {
    let mut current_err = err;
    while let Some(source) = current_err.source() {
        current_err = source;
    }
    current_err
}
