mod home;

use std::io;

use yew::{html, Callback, Context, Html};
use yew_router::{BrowserRouter, Switch};

use crate::{element, router, service, util};

// Public
pub use home::*;

pub enum Message {
    Init(String),
}

pub struct Main {
    base_url: String,
}

impl yew::Component for Main {
    type Message = Message;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        match || -> io::Result<Self> {
            let base_url = web_sys::window()
                .ok_or(io::Error::new(io::ErrorKind::NotFound, "window not found"))?
                .document()
                .ok_or(io::Error::new(
                    io::ErrorKind::NotFound,
                    "document not found",
                ))?
                .base_uri()
                .map_err(util::map_js_error)?
                .ok_or(io::Error::new(
                    io::ErrorKind::NotFound,
                    "base_uri not found",
                ))?;
            Ok(Self { base_url })
        }() {
            Ok(r) => {
                ctx.link().send_future(async {
                    match service::get_version().await {
                        Ok(version) => Self::Message::Init(version),
                        Err(e) => panic!("When fetching version: {e}"),
                    }
                });
                r
            }
            Err(e) => panic!("When accessing base_url: {e}"),
        }
    }

    fn view(&self, _: &Context<Self>) -> Html {
        let mut tree = ("/".to_string(), Vec::new());
        tree.1.push(element::Node::File("painting".to_string()));

        let base_url = self.base_url.clone();
        let menu_switch = Callback::from(move |key: String| {
            match || -> io::Result<()> {
                let location = web_sys::window()
                    .ok_or(io::Error::new(io::ErrorKind::NotFound, "window not found"))?
                    .document()
                    .ok_or(io::Error::new(
                        io::ErrorKind::NotFound,
                        "document not found",
                    ))?
                    .location()
                    .ok_or(io::Error::new(
                        io::ErrorKind::NotFound,
                        "location not found",
                    ))?;
                match key.as_str() {
                    "painting" => location.replace(&format!("{base_url}")),
                    _ => location.replace(&format!("{base_url}404")),
                }
                .map_err(util::map_js_error)?;
                Ok(())
            }() {
                Ok(_) => (),
                Err(e) => panic!("When accessing location: {e}"),
            }
        });

        html! {
            <div class={"main"}>
                <div class={"main-header"}>{"Huiwen"}</div>
                <div class={"main-content"}>
                    <element::Tree {tree} switch={menu_switch} classes={"main-content-menu"} />
                    <BrowserRouter><Switch<router::Route> render={router::switch} /></BrowserRouter>
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
