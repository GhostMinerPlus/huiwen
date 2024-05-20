mod page;

use std::time::Duration;

use yew::{html, Callback, Context, Html};

use crate::{component::Modal, element, err, router, service, util};

fn build_error_modal(ctx: &Context<Main>, e: &err::Error) -> Html {
    match e {
        err::Error::Other(msg) => {
            let link = ctx.link().clone();
            let on_clear_error = Callback::from(move |_| {
                link.send_message(Message::ClearError);
            });
            html! {
                <Modal close={on_clear_error}>
                    <div style={"padding: 1em;background-color: red;width: 100%;"}>{"Error"}</div>
                    <pre style={"padding: 1em;flex: 1;width: 100%;"}>{msg.clone()}</pre>
                </Modal>
            }
        }
        err::Error::NotLogin => html! {
            <element::LoginModal login_uri={"/service/edge/login"} register_uri={"/service/edge/register"} />
        },
    }
}

// Public
pub use page::*;

pub enum Message {
    Init(String),
    Error(err::Error),
    ClearError,
}

pub struct Main {
    base_uri: String,
    err_msg_op: Option<err::Error>,
}

impl yew::Component for Main {
    type Message = Message;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let base_uri = match util::get_base_uri() {
            Ok(rs) => match rs {
                Some(rs) => {
                    ctx.link().send_future(async {
                        match service::get_version().await {
                            Ok(version) => Self::Message::Init(version),
                            Err(e) => Self::Message::Error(e),
                        }
                    });
                    rs
                }
                None => {
                    let e = err::Error::Other(format!("when create:\n\tfailed to get base uri"));
                    ctx.link().send_message(Self::Message::Error(e));
                    String::new()
                }
            },
            Err(e) => {
                ctx.link().send_message(Self::Message::Error(e));
                String::new()
            }
        };
        Self {
            base_uri,
            err_msg_op: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut tree = ("/".to_string(), Vec::new());
        tree.1.push(element::Node::File("painting".to_string()));

        let base_url = self.base_uri.clone();
        let link = ctx.link().clone();
        let menu_switch = Callback::from(move |key: String| {
            let location = match util::get_location() {
                Some(rs) => rs,
                None => {
                    let e = err::Error::Other(format!("when view\n:\t无法获取 location 实例"));
                    link.send_message(Self::Message::Error(e));
                    return;
                }
            };
            if let Err(e) = match key.as_str() {
                "painting" => location.replace(&format!("{base_url}")),
                _ => location.replace(&format!("{base_url}404")),
            } {
                let e = util::map_js_error(e);
                link.send_message(Self::Message::Error(e));
            }
        });

        let link = ctx.link().clone();
        let on_error = Callback::from(move |e: err::Error| {
            link.send_message(Self::Message::Error(e));
        });

        let modal_op = self.err_msg_op.as_ref().map(|e| build_error_modal(ctx, e));

        html! {
            <div class={"main"}>
                <div class={"main-header"}>{"Huiwen"}</div>
                <div class={"main-content"}>
                    <element::Tree {tree} switch={menu_switch} classes={"main-content-menu"} />
                    <router::Router on_error={on_error} />
                </div>
                if modal_op.is_some() {
                    {modal_op.unwrap()}
                }
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Init(version) => {
                log::info!("version: {version}");
                false
            }
            Message::Error(e) => {
                if self.err_msg_op.is_some() {
                    ctx.link().send_future(async {
                        yew::platform::time::sleep(Duration::from_millis(500)).await;
                        Self::Message::Error(e)
                    });
                    false
                } else {
                    log::error!("{e}");
                    self.err_msg_op = Some(e);
                    true
                }
            }
            Message::ClearError => {
                self.err_msg_op = None;
                true
            }
        }
    }
}
