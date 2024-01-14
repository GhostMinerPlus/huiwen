mod home;

use yew::{html, Callback, Context, Html};
use yew_router::{BrowserRouter, Switch};

use crate::{element, router};

// Public
pub use home::*;

pub struct Main {
    base_url: String,
}

impl yew::Component for Main {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let base_url = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .base_uri()
            .unwrap()
            .unwrap();

        Self { base_url }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
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
