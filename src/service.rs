use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

pub(crate) async fn create_user(name: &str, password: &str) -> Result<String, JsValue> {
    let mut req = views::Request::new(&format!("/mirror/system/create_user"));
    req.with_header("content-type", "application/json")
        .with_body(JsValue::from_str(&format!(
            "{{\"id\": \"0\",\"name\": \"{name}\", \"password\": \"{password}\"}}",
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

pub(crate) async fn create_token(name: &str, password: &str) -> Result<(), JsValue> {
    let mut req = views::Request::new(&format!("/mirror/system/create_token"));
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
        Ok(())
    } else {
        Err(JsValue::from_str(result?.as_str()))
    }
}

pub(crate) async fn get_user_name() -> Result<String, JsValue> {
    let req = views::Request::new(&format!("/mirror/system/get_user_name"));
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
