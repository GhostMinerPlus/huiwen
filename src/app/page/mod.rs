pub(crate) mod sign_in;
pub(crate) mod home;

use yew::{Context, Html, Callback, html};
use yew_router::{BrowserRouter, Switch};

use crate::{app::route::*, engine};

pub(crate) struct Main {}

impl yew::Component for Main {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
<<<<<<<< Updated upstream:src/element/mod.rs
        let mut tree = views::Node::new();
========
        let mut tree = engine::Node::new();
>>>>>>>> Stashed changes:src/app/page/mod.rs
        tree.insert("painting".to_string());

        let menu_switch = {
            Callback::from(move |key: String| {
                let location = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .location()
                    .unwrap();
                let _ = match key.as_str() {
                    "painting" => location.replace("/huiwen"),
                    _ => location.replace("/404"),
                };
            })
        };

        html! {
            <div class={"main"}>
                <div class={"main-header"}>{"Huiwen"}</div>
                <div class={"main-content"}>
<<<<<<<< Updated upstream:src/element/mod.rs
                    <views::Menu {tree} switch={menu_switch} />
========
                    <engine::Menu {tree} switch={menu_switch} />
>>>>>>>> Stashed changes:src/app/page/mod.rs
                    <BrowserRouter><Switch<Route> render={switch} /></BrowserRouter>
                </div>
            </div>
        }
    }
}
