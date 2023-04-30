use std::collections::BTreeMap;

use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{RequestInit, RequestMode, Response};

mod components;
pub use components::*;

mod elements;
pub use elements::*;

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

    pub fn with_header(&mut self, k: &str, v: &str) -> &mut Self {
        self.headers.insert(k.to_string(), v.to_string());
        self
    }

    pub fn with_body(&mut self, body: JsValue) -> &mut Self {
        self.body = body;
        self
    }

    pub async fn post(&self) -> Result<Response, JsValue> {
        self.request("POST").await
    }

    pub async fn delete(&self) -> Result<Response, JsValue> {
        self.request("DELETE").await
    }

    pub async fn put(&self) -> Result<Response, JsValue> {
        self.request("PUT").await
    }

    pub async fn get(&self) -> Result<Response, JsValue> {
        self.request("GET").await
    }

    pub async fn request(&self, method: &str) -> Result<Response, JsValue> {
        let mut opts = RequestInit::new();
        opts.method(method);
        opts.mode(RequestMode::Cors);
        opts.body(Some(&self.body));
        let request = web_sys::Request::new_with_str_and_init(&self.url, &opts).unwrap();
        for (k, v) in &self.headers {
            let _ = request.headers().set(&k, &v);
        }
        let promise = web_sys::window().unwrap().fetch_with_request(&request);
        JsFuture::from(promise).await?.dyn_into()
    }
}
