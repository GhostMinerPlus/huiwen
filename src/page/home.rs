use crate::*;

pub struct HomePage {}

impl yew::Component for HomePage {
    type Message = ();

    type Properties = ();

    fn create(_: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &yew::Context<Self>) -> yew::Html {
        yew::html! {
            <div class={"main-content-page"}>
                <element::Canvas />
            </div>
        }
    }
}
