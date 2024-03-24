use std::{cmp::min, io};

use cgmath::Point3;
use painting::point::Point;
use wasm_bindgen_futures::JsFuture;

use crate::util::{self, Request};

fn p3_to_str(pt: &Point3<f32>) -> String {
    format!("{},{},{}", pt.x, pt.y, pt.z)
}

fn str_to_p3(s: &str) -> Point3<f32> {
    let xyz: Vec<f32> = s.split(',').map(|s| s.parse::<f32>().unwrap()).collect();
    Point3 {
        x: xyz[0],
        y: xyz[1],
        z: xyz[2],
    }
}

fn c4_to_str(c4: &[f32; 4]) -> String {
    format!("{},{},{},{}", c4[0], c4[1], c4[2], c4[3])
}

fn str_to_c4(s: &str) -> [f32; 4] {
    let c4: Vec<f32> = s.split(',').map(|s| s.parse::<f32>().unwrap()).collect();
    [c4[0], c4[1], c4[2], c4[3]]
}

async fn execute(script: &str) -> io::Result<Vec<String>> {
    let res = Request::new("/service/edge/execute")
        .with_body_txt(script)?
        .send("POST")
        .await?;
    let rs = JsFuture::from(res.text().map_err(util::map_js_error)?)
        .await
        .map_err(util::map_js_error)?
        .as_string()
        .ok_or(io::Error::new(io::ErrorKind::NotFound, "returned empty"))?;
    Ok(serde_json::from_str(&rs).unwrap())
}

// Public
pub async fn get_version() -> io::Result<String> {
    let rs = execute("$->$output = huiwen->version _").await?;
    Ok(rs[0].clone())
}

pub async fn commit_edge(edge: Vec<Point>) -> io::Result<()> {
    let mut script = format!("$->$edge = ? _");

    for pt in &edge {
        script = format!(
            r#"{script}
$->$point = ? _
$->$point->pos = {} _
$->$point->color = {} _
$->$point->width = {} _
$->$edge->point append $->$edge->point $->$point"#,
            p3_to_str(&pt.pos),
            c4_to_str(&pt.color),
            pt.width
        );
    }
    execute(&format!(
        r#"{script}
huiwen->canvas->edge append huiwen->canvas->edge $->$edge"#
    ))
    .await?;
    Ok(())
}

pub async fn pull_edge_v() -> io::Result<Vec<Vec<Point>>> {
    let mut edge_v = Vec::new();
    let edge_h_v = execute("$->$output = huiwen->canvas->edge _").await?;
    for edge_h in &edge_h_v {
        let mut edge = Vec::new();
        let width_h_v = execute(&format!("$->$output = {edge_h}->point->width _")).await?;
        let color_h_v = execute(&format!("$->$output = {edge_h}->point->color _")).await?;
        let pos_h_v = execute(&format!("$->$output = {edge_h}->point->pos _")).await?;
        let sz = min(width_h_v.len(), min(color_h_v.len(), pos_h_v.len()));
        for i in 0..sz {
            edge.push(Point {
                pos: str_to_p3(&pos_h_v[i]),
                color: str_to_c4(&color_h_v[i]),
                width: width_h_v[i].parse().unwrap(),
            });
        }
        edge_v.push(edge);
    }
    Ok(edge_v)
}

pub async fn clear() -> io::Result<()> {
    let script = format!(
        r#"huiwen->canvas->edge->point->width = _ _
huiwen->canvas->edge->point->color = _ _
huiwen->canvas->edge->point->pos = _ _
huiwen->canvas->edge->point = _ _
huiwen->canvas->edge = _ _"#
    );
    execute(&script).await?;
    Ok(())
}
