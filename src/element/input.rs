use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct InputProps {
    pub name: String,
    /// The controlled value of this form element.
    #[prop_or_default]
    pub value: String,
    /// The callback to be used for propagating changes to this element's value.
    #[prop_or_default]
    pub update: Callback<String>,

    #[prop_or_default]
    pub classes: Classes,
    /// The placeholder value for this component.
    #[prop_or_default]
    pub placeholder: String,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
    // Type
    #[prop_or_default]
    pub r#type: String,
}

pub struct Input {}

impl yew::Component for Input {
    type Message = ();

    type Properties = InputProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let class = classes!("input", props.classes.clone(),);

        let oninput = props.update.reform(|ev: web_sys::InputEvent| {
            let input: HtmlInputElement = ev.target_dyn_into().unwrap();
            input.value()
        });

        html! {
            <input
                name={props.name.clone()}
                value={props.value.clone()}
                {oninput}
                {class}
                type={props.r#type.clone()}
                placeholder={props.placeholder.clone()}
                disabled={props.disabled}
                />
        }
    }
}
