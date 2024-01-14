use std::io;

use cgmath::Point3;
use painting::point::Point;
use wasm_bindgen_futures::JsFuture;

use crate::util::{self, Request};

async fn execute(script: &str) -> io::Result<String> {
    let res = Request::new("/service/edge/execute")
        .with_body_txt(script)?
        .send("POST")
        .await?;
    let rs = JsFuture::from(res.text().map_err(util::map_js_error)?)
        .await
        .map_err(util::map_js_error)?
        .as_string()
        .ok_or(io::Error::new(io::ErrorKind::NotFound, "returned empty"))?;
    Ok(rs)
}

// Public
pub async fn get_version() -> io::Result<String> {
    execute("? return huiwen->version").await
}

pub async fn get_canvas() -> io::Result<String> {
    execute("? return huiwen->canvas").await
}

pub async fn commit_edge(canvas: &str, edge: Vec<Point>) -> io::Result<()> {
    log::info!("commit edge: {canvas}: {:?}", edge);
    // TODO:
    Ok(())
}

pub async fn get_edge_v(canvas: &str) -> io::Result<Vec<Vec<Point>>> {
    log::info!("get edge_v: {canvas}");

    let mut edge_v = Vec::new();
    let mut edge = Vec::new();
    edge.push(Point {
        pos: Point3::new(0.0, 0.0, 0.0),
        color: [0.2, 0.6, 0.0, 1.0],
        width: 0.07,
    });
    edge.push(Point {
        pos: Point3::new(0.0, 0.5, 0.0),
        color: [0.7, 0.0, 0.5, 1.0],
        width: 0.03,
    });
    edge.push(Point {
        pos: Point3::new(0.5, 0.2, 0.0),
        color: [0.0, 0.4, 0.8, 1.0],
        width: 0.1,
    });
    edge_v.push(edge);
    // TODO:
    Ok(edge_v)
}
