use crate::*;

pub enum Msg {
    Nothing,
}

pub struct CanvasPage {}

impl yew::Component for CanvasPage {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        yew::html! {
            <div class={"page"}>
                <element::Canvas />
            </div>
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Nothing => false,
        }
    }
}
