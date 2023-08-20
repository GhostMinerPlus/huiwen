use yew::prelude::*;
use yew_router::prelude::*;

use crate::element::{home::CanvasPage, sign_in::SignInPage};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/huiwen")]
    Home,
    #[at("/huiwen/sign_in")]
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
