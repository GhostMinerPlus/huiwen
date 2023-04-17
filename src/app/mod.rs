mod pages;
mod route;
mod services;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::route::*;

pub(crate) struct App {
    name: String,
    mirror: String,
}

impl App {
    fn new() -> Self {
        Self {
            name: "".to_string(),
            mirror: "http://[2409:8a55:356a:7d81:2ca0:42b3:5937:2]:8080".to_string(),
        }
    }
}

static mut APP: Option<App> = None;

impl App {
    pub(crate) fn start_app() {
        unsafe {
            APP = Some(App::new());
            APP.as_mut().unwrap().run();
        }
    }

    pub(crate) fn get_app() -> &'static App {
        unsafe { APP.as_ref().unwrap() }
    }

    fn run(&mut self) {
        yew::Renderer::<Main>::new().render();
    }
}

pub(crate) struct Main {}

impl yew::Component for Main {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class={"main"}>
                <div class={"header"}>{"Huiwen"}</div>
                <div class={"content"}>
                    <div class={"menu"}>{"这是目录"}</div>
                    <BrowserRouter><Switch<Route> render={switch} /></BrowserRouter>
                </div>
            </div>
        }
    }
}
