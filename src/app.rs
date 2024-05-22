mod page;

use std::time::Duration;

use yew::{html, Callback, Context, Html};

use crate::{component::Modal, element, err, router, service, util};

fn build_error_modal(ctx: &Context<Main>, e: &err::Error) -> Option<Html> {
    let link = ctx.link().clone();
    let on_error = Callback::from(move |e| {
        link.send_message(Message::Error(e));
    });
    match e {
        err::Error::Other(msg) => {
            let link = ctx.link().clone();
            let on_clear_error = Callback::from(move |_| {
                link.send_message(Message::ClearError);
            });
            Some(html! {
                <Modal on_close={on_clear_error}>
                    <div style={"padding: 1em;background-color: red;width: 100%;"}>{"Error"}</div>
                    <pre style={"padding: 1em;flex: 1;width: 100%;"}>{msg.clone()}</pre>
                </Modal>
            })
        }
        err::Error::NotLogin(_) => {
            let link = ctx.link().clone();
            let on_logined = Callback::from(move |_| {
                link.send_message(Message::ClearError);
            });
            let link = ctx.link().clone();
            let on_registered = Callback::from(move |_| {
                link.send_message(Message::Error(err::Error::NotLogin(format!("need login"))));
            });
            Some(html! {
                <element::LoginModal
                    login_uri={"/service/edge/login"}
                    register_uri={"/service/edge/register"}
                    {on_error}
                    {on_logined}
                    {on_registered} />
            })
        }
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

    fn create(_: &Context<Self>) -> Self {
        let base_uri = util::get_base_uri()
            .expect("can not get base uri")
            .expect("can not get base uri");
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
            let location = util::get_location().expect("can not get location");
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

        let modal_op = if let Some(e) = &self.err_msg_op {
            build_error_modal(ctx, e)
        } else {
            None
        };

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
                if let Some(cur_err) = &self.err_msg_op {
                    if e == *cur_err {
                        return true;
                    }
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
                log::info!("clear error");
                true
            }
        }
    }
}
