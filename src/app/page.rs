use painting::point::Point;
use yew::Callback;

use crate::{
    component::{Column, Row},
    element, err, service,
};

#[derive(yew::Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub on_error: Callback<err::Error>,
}

pub enum Message {
    Init(Vec<Vec<Point>>),
    Commit(Vec<Point>),
    Pull(Vec<Vec<Point>>),
    Post(bool),
    PostPull,
    Clear,
    Error(err::Error),
    Bigger,
    Smaller,
}

#[derive(Default)]
pub struct HomePage {
    edge_v: Vec<Vec<Point>>,
    scale: u32,
}

impl yew::Component for HomePage {
    type Message = Message;

    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        ctx.link().send_future(async {
            match service::pull_edge_v().await {
                Ok(r) => Self::Message::Init(r),
                Err(e) => Self::Message::Error(e),
            }
        });
        Self {
            edge_v: Vec::new(),
            scale: 62,
        }
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

        let link = ctx.link().clone();
        let bigger = Callback::from(move |_| {
            link.send_message(Self::Message::Bigger);
        });

        let link = ctx.link().clone();
        let smaller = Callback::from(move |_| {
            link.send_message(Self::Message::Smaller);
        });

        let edge_v = self.edge_v.clone();

        yew::html! {
            <Column
                width={format!("calc(100% - 12.5em)")}
                height={format!("100%")}
                border={format!("1em solid transparent")}
                justify_content={format!("space-between")}>
                <Row height={format!("1.5em")}>
                    <button onclick={pull}>{"Pull"}</button>
                    <button onclick={clear}>{"Clear"}</button>
                    <button onclick={bigger}>{"+"}</button>
                    <button onclick={smaller}>{"-"}</button>
                </Row>
                <Column
                    height={format!("calc(100% - 2em)")}
                    overflow_x={format!("overlay")}
                    overflow_y={format!("overlay")}>
                    <element::Canvas width={1024} height={1024} {commit} {edge_v} />
                </Column>
            </Column>
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
                    match service::pull_edge_v().await {
                        Ok(r) => Self::Message::Pull(r),
                        Err(e) => Self::Message::Error(e),
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
                    match service::clear().await {
                        Ok(_) => Self::Message::Post(false),
                        Err(e) => Self::Message::Error(e),
                    }
                });
                true
            }
            Message::Error(e) => {
                ctx.props().on_error.emit(e);
                false
            }
            Message::Bigger => {
                self.scale += 1;
                false
            }
            Message::Smaller => {
                self.scale -= 1;
                false
            }
        }
    }
}
