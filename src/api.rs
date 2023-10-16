use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

use crate::engine;

pub fn init() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Info).unwrap();
}

pub async fn execute(script: JsValue) -> Result<JsValue, JsValue> {
    let mut req = engine::Request::new(&format!("/mirror/execute"));
    req.with_header("Content-type", "application/json")
        .with_body(script);
    let resp = req.post().await?;
    JsFuture::from(resp.json()?).await
}

pub async fn sign_up(user: &str, password: &str) -> Result<JsValue, JsValue> {
    let mut req = engine::Request::new(&format!("/mirror/signup"));
    req.with_header("Content-type", "application/json")
        .with_body(JsValue::from_str(&format!("{{\"user\":\"{user}\",\"password\":\"{password}\"}}")));
    let resp = req.post().await?;
    JsFuture::from(resp.json()?).await
}

pub async fn check() -> Result<JsValue, JsValue> {
    execute(JsValue::from_str(&format!("[\"add\", \"1\", \"1\"]"))).await
}
