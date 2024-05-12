use yew::{html, Children, Context, Html, Properties};

use crate::util::style_or;

// Public
#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ColumnProps {
    #[prop_or_default]
    pub width: String,
    #[prop_or_default]
    pub height: String,
    #[prop_or_default]
    pub flex: String,
    #[prop_or_default]
    pub border: String,
    #[prop_or_default]
    pub margin: String,
    #[prop_or_default]
    pub padding: String,
    #[prop_or_default]
    pub bk_color: String,
    #[prop_or_default]
    pub overflow_y: String,
    pub children: Children,
}

pub struct Column {}

impl yew::Component for Column {
    type Message = ();

    type Properties = ColumnProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let style = format!(
            "display: flex;flex-direction: column;{}{}{}{}{}{}{}{}",
            style_or("width", &ctx.props().width, None),
            style_or("height", &ctx.props().height, None),
            style_or("flex", &ctx.props().flex, None),
            style_or("border", &ctx.props().border, None),
            style_or("margin", &ctx.props().margin, None),
            style_or("padding", &ctx.props().padding, None),
            style_or("background-color", &ctx.props().bk_color, None),
            style_or("overflow-y", &ctx.props().overflow_y, None),
        );
        html! {
            <div style={style}>
                { ctx.props().children.clone() }
            </div>
        }
    }
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct RowProps {
    #[prop_or_default]
    pub width: String,
    #[prop_or_default]
    pub height: String,
    #[prop_or_default]
    pub flex: String,
    #[prop_or_default]
    pub border: String,
    #[prop_or_default]
    pub margin: String,
    #[prop_or_default]
    pub padding: String,
    #[prop_or_default]
    pub bk_color: String,
    #[prop_or_default]
    pub overflow_x: String,
    pub children: Children,
}

pub struct Row {}

impl yew::Component for Row {
    type Message = ();

    type Properties = RowProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let style = format!(
            "display: flex;flex-direction: row;{}{}{}{}{}{}{}{}",
            style_or("width", &ctx.props().width, None),
            style_or("height", &ctx.props().height, None),
            style_or("flex", &ctx.props().flex, None),
            style_or("border", &ctx.props().border, None),
            style_or("margin", &ctx.props().margin, None),
            style_or("padding", &ctx.props().padding, None),
            style_or("background-color", &ctx.props().bk_color, None),
            style_or("overflow-x", &ctx.props().overflow_x, None),
        );
        html! {
            <div style={style}>
                { ctx.props().children.clone() }
            </div>
        }
    }
}
