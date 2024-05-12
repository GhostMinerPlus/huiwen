use cgmath::Point3;
use painting::point::Point;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::JsFuture;

use crate::{
    err,
    util::{self, Request},
};

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

async fn execute(script_tree: &ScriptTree) -> err::Result<json::JsonValue> {
    let res = Request::new("/service/edge/execute1")
        .with_body_txt(&serde_json::to_string(script_tree).unwrap())?
        .send("POST")
        .await?;
    let rs = JsFuture::from(res.text().map_err(util::map_js_error)?)
        .await
        .map_err(util::map_js_error)?
        .as_string()
        .ok_or(err::Error::Other("returned empty".to_string()))?;
    Ok(json::parse(&rs).unwrap())
}

// Public
#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptTree {
    pub script: String,
    pub name: String,
    pub next_v: Vec<ScriptTree>,
}

pub async fn get_version() -> err::Result<String> {
    let rs = execute(&ScriptTree {
        script: "$->$output = = huiwen->version _".to_string(),
        name: format!("version"),
        next_v: vec![],
    })
    .await?;
    Ok(rs["version"][0].as_str().unwrap().to_string())
}

pub async fn commit_edge(edge: Vec<Point>) -> err::Result<()> {
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
    script = format!(
        r#"{script}
huiwen->canvas->edge += = $->$edge _"#
    );
    execute(&ScriptTree {
        script,
        name: format!(""),
        next_v: vec![],
    })
    .await?;
    Ok(())
}

pub async fn pull_edge_v() -> err::Result<Vec<Vec<Point>>> {
    let r_tree = execute(&ScriptTree {
        script: format!("$->$output = = huiwen->canvas->edge _"),
        name: format!("edge"),
        next_v: vec![
            ScriptTree {
                script: format!("$->$output = = $->$input->point->width _"),
                name: format!("width"),
                next_v: vec![],
            },
            ScriptTree {
                script: format!("$->$output = = $->$input->point->color _"),
                name: format!("color"),
                next_v: vec![],
            },
            ScriptTree {
                script: format!("$->$output = = $->$input->point->pos _"),
                name: format!("pos"),
                next_v: vec![],
            },
        ],
    })
    .await?;

    let mut edge_v = Vec::new();
    let width_h_v2 = &r_tree["edge"]["width"];
    let color_h_v2 = &r_tree["edge"]["color"];
    let pos_h_v2 = &r_tree["edge"]["pos"];
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

pub async fn clear() -> err::Result<()> {
    execute(&ScriptTree {
        script: [
            "huiwen->canvas->edge->point->width = = _ _",
            "huiwen->canvas->edge->point->color = = _ _",
            "huiwen->canvas->edge->point->pos = = _ _",
            "huiwen->canvas->edge->point = = _ _",
            "huiwen->canvas->edge = = _ _",
        ]
        .join("\n"),
        name: format!(""),
        next_v: vec![],
    })
    .await?;
    Ok(())
}
