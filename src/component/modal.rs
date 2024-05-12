use yew::Callback;

use crate::util::{self, style_or};

#[derive(yew::Properties, PartialEq)]
pub struct ModalProps {
    #[prop_or_default]
    pub width: String,
    #[prop_or_default]
    pub height: String,
    #[prop_or_default]
    pub bk_color: String,
    #[prop_or_default]
    pub close: Callback<()>,
    pub children: yew::Children,
}

pub enum Msg {
    Closed,
}

pub struct Modal {
    is_alive: bool,
}

impl yew::Component for Modal {
    type Message = Msg;

    type Properties = ModalProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self { is_alive: true }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let props = ctx.props();

        let onclick = {
            let close = props.close.clone();
            let link = ctx.link().clone();
            Callback::from(move |_| {
                link.send_message(Msg::Closed);
                close.emit(());
            })
        };

        let modal_host = util::get_document()
            .unwrap()
            .get_elements_by_tag_name("body")
            .get_with_index(0)
            .unwrap();

        let style = format!("position: absolute;width: 100%;height: 100%;background-color: #7f7f7f7f;overflow: hide;{}",
            if self.is_alive {
                "display: flex;"
            } else {
                "display: none;"
            });

        yew::create_portal(
            yew::html! {
               <div style={style} {onclick}>
                    <div style={format!("margin: auto auto;{}{}{}",
                        style_or("width", &props.width, None),
                        style_or("height", &props.height, None),
                        style_or("background-color", &props.bk_color, None))}
                        onclick={Callback::from(|e: web_sys::MouseEvent|{e.stop_propagation()})}>
                        {for props.children.iter()}
                    </div>
               </div>
            },
            modal_host,
        )
    }

    fn update(&mut self, _: &yew::prelude::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Closed => {
                self.is_alive = false;
                true
            }
        }
    }
}
