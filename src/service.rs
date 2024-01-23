use std::io;

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
    execute("_ return huiwen->version").await
}

pub async fn get_canvas() -> io::Result<String> {
    execute("_ return huiwen->canvas").await
}

pub async fn commit_edge(canvas: &str, edge: Vec<Point>) -> io::Result<()> {
    let mut script = format!(r#""{canvas}->edge" append ?"#);

    for pt in &edge {
        script = format!(
            r#"{script}
"->point" set ?
"->point->pos" set {}
"->point->color" set {}
"->point->width" set {}
"{canvas}->edge->point" append ->point"#,
            p3_to_str(&pt.pos),
            c4_to_str(&pt.color),
            pt.width
        );
    }
    execute(&script).await?;
    Ok(())
}

pub async fn pull_edge_v(canvas: &str) -> io::Result<Vec<Vec<Point>>> {
    let script = format!(
        r#""->result->root" set {canvas}
"->result->dimension" set edge
"->result->dimension" append point
"->result->attr" set pos
"->result->attr" append color
"->result->attr" append width
"" dump ->result"#
    );
    let s = execute(&script).await?;
    let rs: json::JsonValue = json::parse(&s).unwrap();

    let mut edge_v = Vec::new();
    let mut edge_h = String::new();
    for edge_point_json in rs.members() {
        if edge_h != edge_point_json["edge"].as_str().unwrap() {
            edge_v.push(Vec::new());
            edge_h = edge_point_json["edge"].as_str().unwrap().to_string();
        }
        let pos = edge_point_json["pos"].as_str().unwrap().to_string();
        let color = edge_point_json["color"].as_str().unwrap().to_string();
        let width = edge_point_json["width"].as_str().unwrap().parse().unwrap();
        let pt = Point {
            pos: str_to_p3(&pos),
            color: str_to_c4(&color),
            width,
        };
        edge_v.last_mut().unwrap().push(pt);
    }

    Ok(edge_v)
}
