use yew::Callback;

use crate::component::{self, Row};

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
}

pub struct LoginModal {

}

impl yew::Component for LoginModal {
    type Message = ();

    type Properties = ModalProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &yew::Context<Self>) -> yew::Html {
        yew::html! {
            <component::Modal>
                <div style={"padding: 1em;background-color: green;width: 100%;"}>{"Login"}</div>
                <form style={"width: 100%;flex: 1;"}>
                    <Row width={"100%"}>
                        <label for="username">{"邮箱："}</label>
                        <input style={"flex: 1;"} type="text" name="username" required={true} />
                    </Row>
                    <Row width={"100%"}>
                        <label for="password">{"密码："}</label>
                        <input style={"flex: 1;"} type="password" name="password" required={true} />
                    </Row>
                    <input type="submit" value="登录" />
                </form>
            </component::Modal>
        }
    }
}
