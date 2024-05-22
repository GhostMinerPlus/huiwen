use yew::Callback;

use crate::util;

#[derive(yew::Properties, PartialEq)]
pub struct ModalProps {
    #[prop_or_default]
    pub width: String,
    #[prop_or_default]
    pub height: String,
    #[prop_or_default]
    pub bk_color: String,
    #[prop_or_default]
    pub on_close: Callback<()>,
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
            let close = props.on_close.clone();
            Callback::from(move |_| {
                close.emit(());
            })
        };

        let modal_host = util::get_document()
            .unwrap()
            .get_elements_by_tag_name("body")
            .get_with_index(0)
            .expect("can not get body");

        yew::create_portal(
            yew::html! {
               <div style={"position: absolute;display: flex;width: 100%;height: 100%;background-color: #7f7f7f7f;"} {onclick}>
                    <div style={format!("margin: auto auto;display: flex;flex-direction: column;{}{}{}",
                        util::style_or("width", &props.width, None),
                        util::style_or("height", &props.height, None),
                        util::style_or("background-color", &props.bk_color, Some("white")))}
                        onclick={Callback::from(|e: web_sys::MouseEvent|{e.stop_propagation()})}>
                        {for props.children.iter()}
                    </div>
               </div>
            },
            modal_host,
        )
    }
}
