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

pub async fn pull_edge_v(canvas: &str, last_edge_h: &str) -> io::Result<(String, Vec<Vec<Point>>)> {
    let first = if last_edge_h.is_empty() {
        format!("{canvas}->edge_v->first")
    } else {
        format!("{last_edge_h}->next")
    };
    let script = format!(
        r#""->return->class" set return
"->return->json" set 1
"->edge_v->class" set {first}
"->edge_v->dimension" set 2
"->edge_v->attr" set pos
"->edge_v->attr" append color
"->edge_v->attr" append width
"" ->return ->edge_v"#
    );
    let s = execute(&script).await?;
    let rs: json::JsonValue = json::parse(&s).unwrap();
    let last = rs["last"].as_str().unwrap().to_string();
    let edge_v_json = &rs["json"];

    let mut edge_v = Vec::new();
    for edge_json in edge_v_json.members() {
        let mut edge = Vec::new();
        for point_json in edge_json.members() {
            let pos = point_json["pos"].as_str().unwrap().to_string();
            let color = point_json["color"].as_str().unwrap().to_string();
            let width = point_json["width"].as_str().unwrap().parse().unwrap();
            edge.push(Point {
                pos: str_to_p3(&pos),
                color: str_to_c4(&color),
                width,
            });
        }
        edge_v.push(edge);
    }

    Ok((last, edge_v))
}
