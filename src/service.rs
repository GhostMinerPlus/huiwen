use std::io;

use wasm_bindgen_futures::JsFuture;

use crate::util::{self, Request};

pub async fn get_version() -> io::Result<String> {
    let res = Request::new("/service/edge/execute")
        .with_body_txt("? return huiwen->version")?
        .send("POST")
        .await?;
    let rs = JsFuture::from(res.text().map_err(util::map_js_error)?)
        .await
        .map_err(util::map_js_error)?
        .as_string()
        .ok_or(io::Error::new(io::ErrorKind::NotFound, "version not found"))?;
    Ok(rs)
}
