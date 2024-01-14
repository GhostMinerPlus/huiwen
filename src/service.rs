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
    let mut script = format!(
        "\"->edge\" set ?
\"->o_last\" set {canvas}->edge_v->last
\"{canvas}->edge_v->last\" set ->edge
\"->temp\" set ->o_last
\"->temp\" cmp_str \"\"
_ jump ->temp
_ jump 2
_ jump 3
\"{canvas}->edge_v->first\" set ->edge
_ jump 2
\"->o_last->next\" set ->edge"
    );

    let pt = &edge[0];
    script = format!(
        "{script}
\"->point\" set ?
\"->point->pos\" set {}
\"->point->color\" set {}
\"->point->width\" set {}
\"->edge->first\" set ->point
\"->edge->last\" set ->point",
        p3_to_str(&pt.pos),
        c4_to_str(&pt.color),
        pt.width
    );
    for pt in &edge[1..] {
        script = format!(
            "{script}
\"->point\" set ?
\"->point->pos\" set {}
\"->point->color\" set {}
\"->point->width\" set {}
\"->last_pt\" set ->edge->last
\"->last_pt->next\" set ->point
\"->edge->last\" set ->point",
            p3_to_str(&pt.pos),
            c4_to_str(&pt.color),
            pt.width
        );
    }
    execute(&script).await?;
    Ok(())
}

pub async fn get_edge_v(canvas: &str) -> io::Result<Vec<Vec<Point>>> {
    let mut edge_v = Vec::new();
    let mut edge_h = execute(&format!("_ return {canvas}->edge_v->first")).await?;
    while !edge_h.is_empty() {
        let mut point_h = execute(&format!("_ return {edge_h}->first")).await?;
        let mut edge = Vec::new();
        while !point_h.is_empty() {
            let pos = execute(&format!("_ return {point_h}->pos")).await?;
            let color = execute(&format!("_ return {point_h}->color")).await?;
            let width = execute(&format!("_ return {point_h}->width"))
                .await?
                .parse()
                .unwrap();
            edge.push(Point {
                pos: str_to_p3(&pos),
                color: str_to_c4(&color),
                width,
            });
            point_h = execute(&format!("_ return {point_h}->next")).await?;
        }
        edge_v.push(edge);
        edge_h = execute(&format!("_ return {edge_h}->next")).await?;
    }
    Ok(edge_v)
}
