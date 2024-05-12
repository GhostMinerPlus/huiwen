use painting::point::Point;
use yew::Callback;

use crate::{
    component::{Column, Modal, Row},
    element, service,
};

pub enum Message {
    Init(Vec<Vec<Point>>),
    Commit(Vec<Point>),
    Pull(Vec<Vec<Point>>),
    Post(bool),
    PostPull,
    Clear,
    Error(String),
    ClearError,
}

#[derive(Default)]
pub struct HomePage {
    edge_v: Vec<Vec<Point>>,
    msg: Option<String>,
}

impl yew::Component for HomePage {
    type Message = Message;

    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        ctx.link().send_future(async {
            match service::pull_edge_v().await {
                Ok(r) => Self::Message::Init(r),
                Err(e) => Self::Message::Error(format!("when get canvas\n:\t{e}")),
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

        let link = ctx.link().clone();
        let clear_err = Callback::from(move |_| {
            link.send_message(Self::Message::ClearError);
        });

        let edge_v = self.edge_v.clone();

        yew::html! {
            <Column
                width={format!("100%")}
                height={format!("100%")}
                border={format!("1em solid transparent")}>
                <Row>
                    <button onclick={pull}>{"Pull"}</button>
                    <button onclick={clear}>{"Clear"}</button>
                </Row>
                <element::Canvas {commit} {edge_v} width={format!("100%")} flex={format!("1")} />
                if self.msg.is_some() {
                    <Modal close={clear_err}>
                        <div>{self.msg.clone().unwrap()}</div>
                    </Modal>
                }
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
                        Err(e) => Self::Message::Error(format!("when get canvas\n:\t{e}")),
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
                        Err(e) => Self::Message::Error(format!("when get canvas\n:\t{e}")),
                    }
                });
                true
            }
            Message::Error(msg) => {
                self.msg = Some(msg);
                true
            }
            Message::ClearError => {
                self.msg = None;
                false
            },
        }
    }
}