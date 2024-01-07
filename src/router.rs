use yew::prelude::*;
use yew_router::prelude::*;

use crate::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <page::CanvasPage /> },
        Route::NotFound => html! {
            <div>{"404"}</div>
        },
    }
}
