use yew::prelude::*;
use yew_router::prelude::*;

use super::pages::{CanvasPage, SignInPage};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/sign_in")]
    SignIn,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <CanvasPage /> },
        Route::SignIn => html! { <SignInPage /> },
        Route::NotFound => html! {
            <div>{"???"}</div>
        },
    }
}
