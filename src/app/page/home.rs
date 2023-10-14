use yew_router::prelude::*;

use crate::{api, app::route, engine};

pub(crate) enum Msg {
    Nothing,
    NeedSign,
}

pub(crate) struct CanvasPage {}

impl yew::Component for CanvasPage {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        ctx.link().send_future({
            async move {
                match api::check().await {
                    Ok(_) => Msg::Nothing,
                    Err(_) => Msg::NeedSign,
                }
            }
        });

        Self {}
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        yew::html! {
            <div class={"page"}>
<<<<<<<< Updated upstream:src/element/home.rs
                <views::Canvas />
========
                <engine::Canvas />
>>>>>>>> Stashed changes:src/app/page/home.rs
            </div>
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Nothing => false,
            Msg::NeedSign => {
                let navigator = ctx.link().navigator().unwrap();
                navigator.push(&route::Route::SignIn);
                false
            }
        }
    }
}
