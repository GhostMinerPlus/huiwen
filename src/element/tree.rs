use web_sys::HtmlElement;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum Node {
    Dir((String, Vec<Node>)),
    File(String),
}

#[derive(Clone, Properties, PartialEq)]
pub struct MenuProps {
    pub tree: (String, Vec<Node>),
    pub switch: Callback<String>,
    #[prop_or_default]
    pub classes: Classes,
}

pub struct Tree {}

impl yew::Component for Tree {
    type Message = ();

    type Properties = MenuProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        let onclick = {
            let switch = props.switch.clone();
            Callback::from(move |e: MouseEvent| {
                let key = e.target_dyn_into::<HtmlElement>().unwrap().inner_text();
                switch.emit(key);
            })
        };

        let class = classes!("menu", props.classes.clone(),);
        html! {
            <aside {class}>
                {for props.tree.clone().1.into_iter().map(|node| {
                    html!{<div class={"menu-node"} onclick={onclick.clone()}>{
                        match node {
                            Node::Dir((name, _)) => name,
                            Node::File(file) => file,
                        }
                    }</div>}
                })}
            </aside>
        }
    }
}
