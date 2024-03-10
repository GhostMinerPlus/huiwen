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

async fn execute(script: &str) -> io::Result<json::Array> {
    let res = Request::new("/service/edge/execute")
        .with_body_txt(script)?
        .send("POST")
        .await?;
    let rs = JsFuture::from(res.text().map_err(util::map_js_error)?)
        .await
        .map_err(util::map_js_error)?
        .as_string()
        .ok_or(io::Error::new(io::ErrorKind::NotFound, "returned empty"))?;
    match json::parse(&rs).unwrap() {
        json::JsonValue::Array(arr) => Ok(arr),
        _ => todo!(),
    }
}

// Public
pub async fn get_version() -> io::Result<String> {
    let rs = execute("$ return huiwen->version").await?;
    Ok(rs[0].as_str().unwrap().to_string())
}

pub async fn commit_edge(edge: Vec<Point>) -> io::Result<()> {
    let mut script = format!("$ $edge ?");

    for pt in &edge {
        script = format!(
            r#"{script}
$ clear $point

$ $point ?
$->$point pos {}
$->$point color {}
$->$point width {}
$->$edge point ->$point"#,
            p3_to_str(&pt.pos),
            c4_to_str(&pt.color),
            pt.width
        );
    }
    execute(&format!(
        r#"{script}
huiwen->canvas edge ->$edge"#
    ))
    .await?;
    Ok(())
}

pub async fn pull_edge_v() -> io::Result<Vec<Vec<Point>>> {
    let script = format!(
        r#"$ $path "huiwen->canvas->point"
$ $item pos
$ $item color
$ $item width
$ dump $
$ return $->$result"#
    );
    let rs = execute(&script).await?;
    let s = rs[0].as_str().unwrap();
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

pub async fn clear() -> io::Result<()> {
    let script = format!(
        r#"huiwen clear canvas

huiwen canvas ?
$ $junk ?

$->$junk $code point
$->$junk $source_code edge
$ dc_ns $->$junk

$->$junk clear $code
$->$junk clear source_code
$->$junk $code pos
$->$junk $source_code point
$ dc_ns $->$junk

$->$junk clear $code
$->$junk $code color
$ dc_ns $->$junk

$->$junk clear $code
$->$junk $code width
$ dc_ns $->$junk"#
    );
    execute(&script).await?;
    Ok(())
}
