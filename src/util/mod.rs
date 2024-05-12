use std::collections::BTreeMap;

use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Document, Location, RequestInit, RequestMode, Response};

use crate::err;

// Public
pub fn style_or(name: &str, value: &str, default: Option<&str>) -> String {
    if value.is_empty() {
        match default {
            Some(v) => format!("{name}: {v};"),
            None => String::new(),
        }
    } else {
        format!("{name}: {value};")
    }
}

pub fn map_js_error(e: JsValue) -> err::Error {
    let js_msg = js_sys::Reflect::get(&e, &JsValue::from_str("message")).unwrap();
    let msg = js_msg.as_string().unwrap();
    err::Error::Other(msg)
}

pub fn get_base_uri() -> err::Result<Option<String>> {
    get_document()
        .ok_or(err::Error::Other("failed to get document".to_string()))?
        .base_uri()
        .map_err(map_js_error)
}

pub fn get_document() -> Option<Document> {
    web_sys::window()?.document()
}

pub fn get_location() -> Option<Location> {
    get_document()?.location()
}

pub struct Request {
    url: String,
    body: JsValue,
    header_v: BTreeMap<String, String>,
}

impl Request {
    pub fn new(url: &str) -> Self {
        Self {
            url: String::from(url),
            body: JsValue::NULL,
            header_v: BTreeMap::new(),
        }
    }

    /// Set header of this request.
    pub fn with_header(&mut self, k: &str, v: &str) -> &mut Self {
        self.header_v.insert(k.to_string(), v.to_string());
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
    pub fn with_body_str(&mut self, body: &str) -> err::Result<&mut Self> {
        self.body = js_sys::JSON::parse(body).map_err(map_js_error)?;
        Ok(self)
    }

    pub fn with_body_txt(&mut self, body: &str) -> err::Result<&mut Self> {
        self.body = JsValue::from_str(body);
        Ok(self)
    }

    pub async fn send(&self, method: &str) -> err::Result<Response> {
        let mut opts = RequestInit::new();
        opts.method(method);
        opts.mode(RequestMode::Cors);
        opts.body(Some(&self.body));
        let request =
            web_sys::Request::new_with_str_and_init(&self.url, &opts).map_err(map_js_error)?;
        for (k, v) in &self.header_v {
            let _ = request.headers().set(&k, &v);
        }
        let promise = web_sys::window()
            .ok_or(err::Error::Other("window not found".to_string()))?
            .fetch_with_request(&request);
        JsFuture::from(promise)
            .await
            .map_err(map_js_error)?
            .dyn_into()
            .map_err(map_js_error)
    }
}
