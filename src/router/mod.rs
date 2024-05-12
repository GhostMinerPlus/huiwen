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

#[derive(yew::Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub on_error: Callback<err::Error>,
}

// Public
pub struct Router {}

impl yew::Component for Router {
    type Message = ();
    type Properties = Props;

    fn create(_: &yew::prelude::Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &yew::prelude::Context<Self>) -> Html {
        let on_error = ctx.props().on_error.clone();
        let switch = Callback::from(move |route: Route| match route {
            Route::Home => html! { <app::HomePage on_error={on_error.clone()} /> },
            Route::NotFound => html! {
                <div>{"404"}</div>
            },
        });

        yew::html! {
            <yew_router::BrowserRouter>
                <yew_router::Switch<Route> render={switch} />
            </yew_router::BrowserRouter>
        }
    }
}
