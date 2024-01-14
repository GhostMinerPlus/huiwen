use std::{collections::BTreeMap, io};

use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{RequestInit, RequestMode, Response};

// Public
pub fn map_js_error(e: JsValue) -> io::Error {
    let js_msg = js_sys::Reflect::get(&e, &JsValue::from_str("message")).unwrap();
    let msg = js_msg.as_string().unwrap();
    io::Error::new(io::ErrorKind::Other, msg)
}

pub struct Request {
    url: String,
    body: JsValue,
    headers: BTreeMap<String, String>,
}

impl Request {
    pub fn new(url: &str) -> Self {
        Self {
            url: String::from(url),
            body: JsValue::NULL,
            headers: BTreeMap::new(),
        }
    }

    /// Set header of this request.
    pub fn with_header(&mut self, k: &str, v: &str) -> &mut Self {
        self.headers.insert(k.to_string(), v.to_string());
        self
    }

    /// Set body of this request with a &[JsValue].
    pub fn with_body(&mut self, body: JsValue) -> &mut Self {
        self.body = body;
        self
    }

    /// Set body of this request with a &[str].
    ///
    /// This method will parse the body in &[str] into [JsValue], so it may return a [Err] when failing to parse.
    pub fn with_body_str(&mut self, body: &str) -> io::Result<&mut Self> {
        self.body = js_sys::JSON::parse(body).map_err(map_js_error)?;
        Ok(self)
    }

    pub fn with_body_txt(&mut self, body: &str) -> io::Result<&mut Self> {
        self.body = JsValue::from_str(body);
        Ok(self)
    }

    pub async fn send(&self, method: &str) -> io::Result<Response> {
        let mut opts = RequestInit::new();
        opts.method(method);
        opts.mode(RequestMode::Cors);
        opts.body(Some(&self.body));
        let request =
            web_sys::Request::new_with_str_and_init(&self.url, &opts).map_err(map_js_error)?;
        for (k, v) in &self.headers {
            let _ = request.headers().set(&k, &v);
        }
        let promise = web_sys::window()
            .ok_or(io::Error::new(io::ErrorKind::NotFound, "window not found"))?
            .fetch_with_request(&request);
        JsFuture::from(promise)
            .await
            .map_err(map_js_error)?
            .dyn_into()
            .map_err(map_js_error)
    }
}
