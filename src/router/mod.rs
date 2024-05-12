use yew::prelude::*;
use yew_router::prelude::*;

use crate::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <app::HomePage /> },
        Route::NotFound => html! {
            <div>{"404"}</div>
        },
    }
}

// Public
pub struct Router {}

impl yew::Component for Router {
    type Message = ();
    type Properties = ();

    fn create(_: &yew::prelude::Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &yew::prelude::Context<Self>) -> Html {
        yew::html! {
            <yew_router::BrowserRouter>
                <yew_router::Switch<Route> render={switch} />
            </yew_router::BrowserRouter>
        }
    }
}
