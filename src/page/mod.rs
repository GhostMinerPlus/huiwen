mod home;

use std::io;

use yew::{html, Callback, Context, Html};
use yew_router::{BrowserRouter, Switch};

use crate::{element, router, util};

// Public
pub use home::*;

pub struct Main {
    base_url: String,
}

impl yew::Component for Main {
    type Message = ();

    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        match || -> io::Result<String> {
            web_sys::window()
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
                ))
        }() {
            Ok(base_url) => Self { base_url },
            Err(e) => {
                panic!("When accessing base_url: {e}");
            }
        }
    }

    fn view(&self, _: &Context<Self>) -> Html {
        let mut tree = ("/".to_string(), Vec::new());
        tree.1.push(element::Node::File("painting".to_string()));

        let base_url = self.base_url.clone();
        let menu_switch = {
            Callback::from(move |key: String| {
                let location = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .location()
                    .unwrap();
                let _ = match key.as_str() {
                    "painting" => location.replace(&format!("{base_url}")),
                    _ => location.replace(&format!("{base_url}404")),
                };
            })
        };

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
}
