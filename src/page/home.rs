use std::io;

use painting::point::Point;
use yew::Callback;

use crate::*;

pub enum Message {
    Init(Vec<Vec<Point>>),
    Commit(Vec<Point>),
    Pull(Vec<Vec<Point>>),
    Post(bool),
    PostPull,
    Clear,
}

#[derive(Default)]
pub struct HomePage {
    edge_v: Vec<Vec<Point>>,
}

impl yew::Component for HomePage {
    type Message = Message;

    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        ctx.link().send_future(async {
            let rs: io::Result<Vec<Vec<Point>>> = async {
                let edge_v = service::pull_edge_v().await?;
                Ok(edge_v)
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
            Message::Init(edge_v) => {
                self.edge_v = edge_v;
                true
            }
            Message::Commit(edge) => {
                ctx.link().send_future(async move {
                    let _ = service::commit_edge(edge).await;
                    Message::Post(false)
                });
                false
            }
            Message::Post(b) => b,
            Message::PostPull => {
                ctx.link().send_future(async move {
                    let rs: io::Result<Vec<Vec<Point>>> = async {
                        let edge_v = service::pull_edge_v().await?;
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
                ctx.link().send_future(async move {
                    let rs: io::Result<()> = async { service::clear().await }.await;
                    match rs {
                        Ok(_) => Self::Message::Post(false),
                        Err(e) => panic!("When clear canvas: {e}"),
                    }
                });
                true
            }
        }
    }
}
