use js_sys::encode_uri_component;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

use super::App;

pub(crate) async fn create_user(name: &str, password: &str) -> Result<String, JsValue> {
    let mut req = views::Request::new(&format!(
        "service/{}/mirror/system/create_user",
        encode_uri_component(&App::get_app().mirror)
    ));
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

pub(crate) fn create_token_url(url: &str) -> String {
    format!(
        "service/{}/mirror/system/create_token/{}",
        encode_uri_component(&App::get_app().mirror),
        encode_uri_component(url)
    )
}

pub(crate) async fn create_token(user_id: &str, password: &str) -> Result<String, JsValue> {
    let mut req = views::Request::new(&format!(
        "service/{}/mirror/system/create_token",
        encode_uri_component(&App::get_app().mirror)
    ));
    req.with_header("content-type", "application/json")
        .with_body(JsValue::from_str(&format!(
            "{{\"id\": \"{user_id}\",\"name\": \"\", \"password\": \"{password}\"}}",
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

pub(crate) async fn get_user_id() -> Result<String, JsValue> {
    let req = views::Request::new(&format!(
        "service/{}/mirror/system/get_user_id",
        encode_uri_component(&App::get_app().mirror)
    ));
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
