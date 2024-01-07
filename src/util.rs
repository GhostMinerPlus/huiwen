use std::{collections::BTreeMap, io};

use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{RequestInit, RequestMode, Response};

async fn request(req: &Request, method: &str) -> io::Result<Response> {
    let mut opts = RequestInit::new();
    opts.method(method);
    opts.mode(RequestMode::Cors);
    opts.body(Some(&req.body));
    let request = web_sys::Request::new_with_str_and_init(&req.url, &opts).unwrap();
    for (k, v) in &req.headers {
        let _ = request.headers().set(&k, &v);
    }
    let promise = web_sys::window().unwrap().fetch_with_request(&request);
    JsFuture::from(promise)
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, ""))?
        .dyn_into()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, ""))
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

    pub fn with_header(&mut self, k: &str, v: &str) -> &mut Self {
        self.headers.insert(k.to_string(), v.to_string());
        self
    }

    pub fn with_body(&mut self, body: JsValue) -> &mut Self {
        self.body = body;
        self
    }

    pub async fn post(&self) -> io::Result<Response> {
        request(self, "POST").await
    }

    pub async fn delete(&self) -> io::Result<Response> {
        request(self, "DELETE").await
    }

    pub async fn put(&self) -> io::Result<Response> {
        request(self, "PUT").await
    }

    pub async fn get(&self) -> io::Result<Response> {
        request(self, "GET").await
    }
}
