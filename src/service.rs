use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

pub(crate) async fn user_create(name: &str, password: &str) -> Result<String, JsValue> {
    let mut req = views::Request::new(&format!("/portal/user/create"));
    req.with_header("content-type", "application/json")
        .with_body(JsValue::from_str(&format!(
            "{{\"name\": \"{name}\", \"password\": \"{password}\"}}",
        )));
    let resp = req.post().await?;
    let result = JsFuture::from(resp.text()?)
        .await?
        .as_string()
        .ok_or(JsValue::from_str("null"));
    if resp.status() == 200 {
        result
    } else {
        Err(JsValue::from_str(result?.as_str()))
    }
}

pub(crate) async fn user_name() -> Result<String, JsValue> {
    let req = views::Request::new(&format!("/portal/user/check"));
    let resp = req.get().await?;
    let result = JsFuture::from(resp.text()?)
        .await?
        .as_string()
        .ok_or(JsValue::from_str("null"));
    if resp.status() == 200 {
        result
    } else {
        Err(JsValue::from_str(result?.as_str()))
    }
}
