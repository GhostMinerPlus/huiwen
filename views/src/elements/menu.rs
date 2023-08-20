use std::collections::HashMap;

use web_sys::HtmlElement;
use yew::prelude::*;

#[derive(Debug)]
pub struct Node {
    tree: HashMap<String, Node>,
}

impl Node {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_node(&mut self, key: String, value: Node) -> Option<Node> {
        self.tree.insert(key, value)
    }

    pub fn insert(&mut self, key: String) -> Option<Node> {
        self.tree.insert(key, Self::default())
    }

    pub fn get(&self, key: &str) -> Option<&Node> {
        self.tree.get(key)
    }

    pub fn len(&self) -> usize {
        self.tree.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }

    pub fn into_iter(self) -> std::collections::hash_map::IntoIter<String, Node> {
        self.tree.into_iter()
    }
}

impl Default for Node {
    fn default() -> Self {
        Self {
            tree: Default::default(),
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.tree == other.tree
    }
}

impl Eq for Node {}

impl Clone for Node {
    fn clone(&self) -> Self {
        Self {
            tree: self.tree.clone(),
        }
    }
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MenuProps {
    pub tree: Node,
    pub switch: Callback<String>,
    #[prop_or_default]
    pub classes: Classes,
}

pub struct Menu {}

impl yew::Component for Menu {
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
                {for props.tree.clone().into_iter().map(|(k, _)| {
                    html!{<div class={"node"} onclick={onclick.clone()}>{k}</div>}
                })}
            </aside>
        }
    }
}
