mod page;

use yew::{html, Callback, Context, Html};

use crate::{element, router, service, util};

// Public
pub use page::*;

pub enum Message {
    Init(String),
}

pub struct Main {
    base_uri: String,
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
                            Err(e) => panic!("When fetching version: {e}"),
                        }
                    });
                    rs
                }
                None => panic!("when create:\n\tfailed to get base uri"),
            },
            Err(e) => panic!("when create:\n{e}"),
        };
        Self { base_uri }
    }

    fn view(&self, _: &Context<Self>) -> Html {
        let mut tree = ("/".to_string(), Vec::new());
        tree.1.push(element::Node::File("painting".to_string()));

        let base_url = self.base_uri.clone();
        let menu_switch = Callback::from(move |key: String| {
            let location = match util::get_location() {
                Some(rs) => rs,
                None => {
                    log::warn!("when view\n:\t无法获取 location 实例");
                    return;
                }
            };
            if let Err(e) = match key.as_str() {
                "painting" => location.replace(&format!("{base_url}")),
                _ => location.replace(&format!("{base_url}404")),
            } {
                let e = util::map_js_error(e);
                log::warn!("when view\n:\t{e}");
            }
        });

        html! {
            <div class={"main"}>
                <div class={"main-header"}>{"Huiwen"}</div>
                <div class={"main-content"}>
                    <element::Tree {tree} switch={menu_switch} classes={"main-content-menu"} />
                    <router::Router />
                </div>
            </div>
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Init(version) => {
                log::info!("version: {version}");
                false
            }
        }
    }
}
