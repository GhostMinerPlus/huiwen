use std::io;

use painting::point::Point;
use yew::Callback;

use crate::*;

pub enum Message {
    Init((String, Vec<Vec<Point>>)),
    Commit(Vec<Point>),
    Pull(Vec<Vec<Point>>),
    Post(bool),
    PostPull,
    Clear,
}

#[derive(Default)]
pub struct HomePage {
    canvas: String,
    edge_v: Vec<Vec<Point>>,
}

impl yew::Component for HomePage {
    type Message = Message;

    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        ctx.link().send_future(async {
            let rs: io::Result<(String, Vec<Vec<Point>>)> = async {
                let canvas = service::get_canvas().await?;
                let edge_v = service::pull_edge_v(&canvas).await?;
                Ok((canvas, edge_v))
            }
            .await;
            match rs {
                Ok(r) => Self::Message::Init(r),
                Err(e) => panic!("When get canvas: {e}"),
            }
        });
        Self::default()
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let link = ctx.link().clone();
        let commit = Callback::from(move |pt_v| {
            link.send_message(Self::Message::Commit(pt_v));
        });

        let link = ctx.link().clone();
        let pull = Callback::from(move |_| {
            link.send_message(Self::Message::PostPull);
        });

        let link = ctx.link().clone();
        let clear = Callback::from(move |_| {
            link.send_message(Self::Message::Clear);
        });

        let edge_v = self.edge_v.clone();

        yew::html! {
            <div class={"main-content-page"}>
                <button onclick={pull}>{"Pull"}</button>
                <button onclick={clear}>{"Clear"}</button>
                <element::Canvas {commit} {edge_v} />
            </div>
        }
    }

    fn update(&mut self, ctx: &yew::prelude::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Init((canvas, edge_v)) => {
                self.canvas = canvas;
                self.edge_v = edge_v;
                true
            }
            Message::Commit(edge) => {
                let canvas = self.canvas.clone();
                ctx.link().send_future(async move {
                    let _ = service::commit_edge(&canvas, edge).await;
                    Message::Post(false)
                });
                false
            }
            Message::Post(b) => b,
            Message::PostPull => {
                let canvas = self.canvas.clone();
                ctx.link().send_future(async move {
                    let rs: io::Result<Vec<Vec<Point>>> = async {
                        let edge_v = service::pull_edge_v(&canvas).await?;
                        Ok(edge_v)
                    }
                    .await;
                    match rs {
                        Ok(r) => Self::Message::Pull(r),
                        Err(e) => panic!("When get canvas: {e}"),
                    }
                });
                false
            }
            Message::Pull(edge_v) => {
                self.edge_v = edge_v;
                true
            }
            Message::Clear => {
                self.edge_v.clear();
                true
            }
        }
    }
}
