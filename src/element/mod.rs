pub(crate) mod sign_in;
pub(crate) mod home;

use yew::{Context, Html, Callback, html};
use yew_router::{BrowserRouter, Switch};

use crate::route::*;

pub(crate) struct Main {}

impl yew::Component for Main {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let mut tree = views::Node::new();
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
                    <views::Menu {tree} switch={menu_switch} />
                    <BrowserRouter><Switch<Route> render={switch} /></BrowserRouter>
                </div>
            </div>
        }
    }
}
