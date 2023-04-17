mod button;
pub use button::*;

#[derive(yew::Properties, PartialEq)]
pub struct ModalProps {
    #[prop_or_default]
    pub classes: yew::Classes,
    #[prop_or_default]
    pub children: yew::Children,
    #[prop_or_default]
    pub r#ref: yew::NodeRef,
}

pub struct Modal {}

impl yew::Component for Modal {
    type Message = ();

    type Properties = ModalProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let modal_host = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_elements_by_tag_name("main")
            .get_with_index(0)
            .unwrap();

        let props = ctx.props();
        yew::create_portal(
            yew::html! {
               <div ref={props.r#ref.clone()} class={props.classes.clone()}>
                   {for props.children.iter()}
               </div>
            },
            modal_host.into(),
        )
    }
}
