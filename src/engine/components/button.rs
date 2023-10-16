use yew::events::MouseEvent;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// The click handler to use for this component.
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let class = classes!("button", props.classes.clone(),);
    html! {
        <button {class} onclick={props.onclick.clone()}>
            {props.children.clone()}
        </button>
    }
}
