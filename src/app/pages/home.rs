use yew_router::prelude::*;

use crate::app::{route, services};

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
                if let Err(_) = services::get_user_id().await {
                    return Msg::NeedSign;
                }
                Msg::Nothing
            }
        });
        Self {}
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        yew::html! {
            <div class={"page"}>
                <huiwen::Canvas />
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
