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

async fn execute(script_tree: json::JsonValue) -> io::Result<json::JsonValue> {
    let res = Request::new("/service/edge/execute")
        .with_body_txt(&json::stringify(script_tree))?
        .send("POST")
        .await?;
    let rs = JsFuture::from(res.text().map_err(util::map_js_error)?)
        .await
        .map_err(util::map_js_error)?
        .as_string()
        .ok_or(io::Error::new(io::ErrorKind::NotFound, "returned empty"))?;
    Ok(json::parse(&rs).unwrap())
}

// Public
pub async fn get_version() -> io::Result<String> {
    let root = "$->$output = = huiwen->version _".to_string();
    let mut script_tree = json::object! {};
    let _ = script_tree.insert(&root, json::Null);
    let rs = execute(script_tree).await?;
    Ok(rs[&root][0].as_str().unwrap().to_string())
}

pub async fn commit_edge(edge: Vec<Point>) -> io::Result<()> {
    let mut script = format!("$->$edge = = ? _");

    for pt in &edge {
        script = format!(
            r#"{script}
$->$point = = ? _
$->$point->pos = = {} _
$->$point->color = = {} _
$->$point->width = = {} _
$->$edge->point += = $->$point _"#,
            p3_to_str(&pt.pos),
            c4_to_str(&pt.color),
            pt.width
        );
    }
    let root = format!(
        r#"{script}
huiwen->canvas->edge += = $->$edge _"#
    );
    let mut script_tree = json::object! {};
    let _ = script_tree.insert(&root, json::Null);
    execute(script_tree).await?;
    Ok(())
}

pub async fn pull_edge_v() -> io::Result<Vec<Vec<Point>>> {
    let mut script_tree = json::object! {};
    // $->$output = huiwen->canvas->edge _
    let huiwen_canvas_edge = {
        let mut huiwen_canvas_edge = json::object! {};
        // $->$output = $->$input->point->width _
        let _ = huiwen_canvas_edge.insert("$->$output = = $->$input->point->width _", json::Null);
        // $->$output = $->$input->point->color _
        let _ = huiwen_canvas_edge.insert("$->$output = = $->$input->point->color _", json::Null);
        // $->$output = $->$input->point->pos _
        let _ = huiwen_canvas_edge.insert("$->$output = = $->$input->point->pos _", json::Null);
        huiwen_canvas_edge
    };
    let _ = script_tree.insert("$->$output = = huiwen->canvas->edge _", huiwen_canvas_edge);

    let r_tree = execute(script_tree).await?;

    let mut edge_v = Vec::new();
    let width_h_v2 =
        &r_tree["$->$output = = huiwen->canvas->edge _"]["$->$output = = $->$input->point->width _"];
    let color_h_v2 =
        &r_tree["$->$output = = huiwen->canvas->edge _"]["$->$output = = $->$input->point->color _"];
    let pos_h_v2 =
        &r_tree["$->$output = = huiwen->canvas->edge _"]["$->$output = = $->$input->point->pos _"];
    for i in 0..width_h_v2.len() {
        let mut edge = Vec::new();
        let width_h_v = &width_h_v2[i];
        let color_h_v = &color_h_v2[i];
        let pos_h_v = &pos_h_v2[i];
        for j in 0..width_h_v.len() {
            let width_h = width_h_v[j].as_str().unwrap();
            let color_h = color_h_v[j].as_str().unwrap();
            let pos_h = pos_h_v[j].as_str().unwrap();
            edge.push(Point {
                pos: str_to_p3(pos_h),
                color: str_to_c4(color_h),
                width: width_h.parse().unwrap(),
            });
        }
        edge_v.push(edge);
    }
    Ok(edge_v)
}

pub async fn clear() -> io::Result<()> {
    let root = format!(
        r#"huiwen->canvas->edge->point->width = = _ _
huiwen->canvas->edge->point->color = = _ _
huiwen->canvas->edge->point->pos = = _ _
huiwen->canvas->edge->point = = _ _
huiwen->canvas->edge = = _ _"#
    );
    let mut script_tree = json::object! {};
    let _ = script_tree.insert(&root, json::Null);
    execute(script_tree).await?;
    Ok(())
}
