use wasm_bindgen::UnwrapThrowExt;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct InputProps {
    /// The controlled value of this form element.
    #[prop_or_default]
    pub value: String,
    /// The callback to be used for propagating changes to this element's value.
    pub update: Callback<String>,

    #[prop_or_default]
    pub classes: Classes,
    /// The placeholder value for this component.
    #[prop_or_default]
    pub placeholder: String,
    /// Use rounded appearance.
    #[prop_or_default]
    pub rounded: bool,
    /// Display a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
    /// Make this component read-only.
    #[prop_or_default]
    pub readonly: bool,
    /// Make this component static.
    #[prop_or_default]
    pub r#static: bool,
}

/// A text input element.
///
/// [https://bulma.io/documentation/form/input/](https://bulma.io/documentation/form/input/)
///
/// All YBC form components are controlled components. This means that the value of the field must
/// be provided from a parent component, and changes to this component are propagated to the parent
/// component via callback.
#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let class = classes!(
        "input",
        props.classes.clone(),
        props.rounded.then_some("is-rounded"),
        props.loading.then_some("is-loading"),
        props.r#static.then_some("is-static"),
    );
    let oninput = props.update.reform(|ev: web_sys::InputEvent| {
        let input: HtmlInputElement = ev
            .target_dyn_into()
            .expect_throw("event target should be an input");
        input.value()
    });
    html! {
        <input
            value={props.value.clone()}
            {oninput}
            {class}
            placeholder={props.placeholder.clone()}
            disabled={props.disabled}
            readonly={props.readonly}
            />
    }
}
