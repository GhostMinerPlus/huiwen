use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

use crate::app;

pub struct Api {}

impl Api {
    pub fn new() -> Self {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Info).unwrap();

        Self {}
    }
}

impl moon::AsApi for Api {
    fn call(&self, mut arg_v: json::JsonValue) -> Result<json::JsonValue, json::JsonError> {
        let tree = arg_v.array_remove(0);
        match tree.as_str().unwrap() {
            "watch" => Ok(json::object! {
                    "0": {
                        "0": {
                            "width": "0.01",
                            "pos": {
                                "x": "0.0",
                                "y": "0.0",
                                "z": "0.0",
                            },
                            "color": {
                                "r": "0.0",
                                "g": "0.0",
                                "b": "0.0",
                                "a": "1.0",
                            }
                        },
                        "1": {
                            "width": "0.02",
                            "pos": {
                                "x": "1.0",
                                "y": "1.0",
                                "z": "0.0",
                            },
                            "color": {
                                "r": "0.2",
                                "g": "0.7",
                                "b": "0.0",
                                "a": "1.0",
                            }
                        }
                    }
                }),
            _ => {
                log::info!("[{tree}, {arg_v}]");
                Ok(json::JsonValue::Null)
            }
        }
    }
}

pub async fn execute(script: JsValue) -> Result<JsValue, JsValue> {
    let mut req = app::Request::new(&format!("/mirror/execute"));
    req.with_header("Content-type", "application/json")
        .with_body(script);
    let resp = req.post().await?;
    JsFuture::from(resp.json()?).await
}

pub async fn sign_up(user: &str, password: &str) -> Result<JsValue, JsValue> {
    let mut req = app::Request::new(&format!("/mirror/signup"));
    req.with_header("Content-type", "application/json")
        .with_body(JsValue::from_str(&format!(
            "{{\"user\":\"{user}\",\"password\":\"{password}\"}}"
        )));
    let resp = req.post().await?;
    JsFuture::from(resp.json()?).await
}

pub async fn check() -> Result<JsValue, JsValue> {
    execute(JsValue::from_str(&format!("[\"add\", \"1\", \"1\"]"))).await
}
