mod page;

use yew::{html, Callback, Context, Html};

use crate::{component::Modal, element, err, router, service, util};

// Public
pub use page::*;

pub enum Message {
    Init(String),
    Error(err::Error),
}

pub struct Main {
    base_uri: String,
    msg: Option<String>,
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
            msg: None,
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

        html! {
            <div class={"main"}>
                <div class={"main-header"}>{"Huiwen"}</div>
                <div class={"main-content"}>
                    <element::Tree {tree} switch={menu_switch} classes={"main-content-menu"} />
                    <router::Router on_error={on_error} />
                </div>
                if self.msg.is_some() {
                    <Modal>
                        <div style={"padding: 1em;"}>{self.msg.clone().unwrap()}</div>
                    </Modal>
                }
            </div>
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Init(version) => {
                log::info!("version: {version}");
                false
            }
            Message::Error(e) => {
                self.msg = Some(e.to_string());
                true
            }
        }
    }
}
