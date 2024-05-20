use web_sys::HtmlInputElement;
use yew::{Callback, MouseEvent, TargetCast};

use crate::{
    component::{self, Row},
    err, util,
};

pub enum Msg {
    SetEmail(String),
    SetPassword(String),
    PostLogin,
    PostRegister,
    Login(err::Result<()>),
    Register(err::Result<()>),
}

#[derive(yew::Properties, PartialEq)]
pub struct ModalProps {
    #[prop_or_default]
    pub width: String,
    #[prop_or_default]
    pub height: String,
    #[prop_or_default]
    pub bk_color: String,
    #[prop_or_default]
    pub login_uri: String,
    #[prop_or_default]
    pub register_uri: String,
}

pub struct LoginModal {
    email: String,
    password: String,
}

impl yew::Component for LoginModal {
    type Message = Msg;

    type Properties = ModalProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            email: String::new(),
            password: String::new(),
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let link = ctx.link().clone();
        let on_set_email = Callback::from(move |ev: web_sys::InputEvent| {
            let input = ev.target_dyn_into::<HtmlInputElement>().unwrap();
            link.send_message(Msg::SetEmail(input.value()));
        });

        let link = ctx.link().clone();
        let on_set_password = Callback::from(move |ev: web_sys::InputEvent| {
            let input = ev.target_dyn_into::<HtmlInputElement>().unwrap();
            link.send_message(Msg::SetPassword(input.value()));
        });

        let link = ctx.link().clone();
        let on_register = Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            link.send_message(Msg::PostRegister);
        });

        let link = ctx.link().clone();
        let on_login = Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            link.send_message(Msg::PostLogin);
        });

        yew::html! {
            <component::Modal>
                <div style={"padding: 1em;background-color: green;width: 100%;"}>{"Login/Register"}</div>
                <form style={"width: 100%;flex: 1;padding: 1em;"}>
                    <Row width={"100%"}>
                        <label for="email">{"邮箱："}</label>
                        <input style={"flex: 1;"} type="text" name="username" required={true} oninput={on_set_email} />
                    </Row>
                    <Row width={"100%"}>
                        <label for="password">{"密码："}</label>
                        <input style={"flex: 1;"} type="password" name="password" required={true} oninput={on_set_password} />
                    </Row>
                    <Row width={"100%"}>
                        <input type="submit" value="注册" onclick={on_register} />
                        <input type="submit" value="登录" onclick={on_login} />
                    </Row>
                </form>
            </component::Modal>
        }
    }

    fn update(&mut self, ctx: &yew::prelude::Context<Self>, msg: Msg) -> bool {
        match msg {
            Msg::SetEmail(email) => {
                self.email = email;
                false
            }
            Msg::SetPassword(password) => {
                self.password = password;
                false
            }
            Msg::PostLogin => {
                let email = self.email.clone();
                let password = self.password.clone();
                let uri = ctx.props().login_uri.clone();
                ctx.link().send_future(async move {
                    match util::Request::new(&uri)
                        .with_body_str(&format!(
                            "{{\"email\":\"{email}\",\"password\":\"{password}\"}}"
                        ))
                        .unwrap()
                        .send("POST")
                        .await
                    {
                        Ok(_) => Msg::Login(Ok(())),
                        Err(e) => Msg::Login(Err(e)),
                    }
                });
                false
            }
            Msg::PostRegister => {
                let email = self.email.clone();
                let password = self.password.clone();
                let uri = ctx.props().register_uri.clone();
                ctx.link().send_future(async move {
                    match util::Request::new(&uri)
                        .with_body_str(&format!(
                            "{{\"email\":\"{email}\",\"password\":\"{password}\"}}"
                        ))
                        .unwrap()
                        .send("POST")
                        .await
                    {
                        Ok(_) => Msg::Register(Ok(())),
                        Err(e) => Msg::Register(Err(e)),
                    }
                });
                false
            }
            Msg::Login(rs) => true,
            Msg::Register(rs) => true,
        }
    }
}
