use yew::{classes, Callback};

#[derive(yew::Properties, PartialEq)]
pub struct ModalProps {
    pub close: Callback<()>,

    #[prop_or_default]
    pub classes: yew::Classes,
    #[prop_or_default]
    pub children: yew::Children,
}

pub struct Modal {}

impl yew::Component for Modal {
    type Message = ();

    type Properties = ModalProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let props = ctx.props();

        let onclick = {
            let close = props.close.clone();
            Callback::from(move |_| {
                close.emit(());
            })
        };

        yew::html! {
           <div class={"modal-bk"} {onclick}>
                <div class={classes!("box", props.classes.clone())} onclick={Callback::from(|e: web_sys::MouseEvent|{e.stop_propagation()})}>
                    {for props.children.iter()}
                </div>
           </div>
        }
    }
}
