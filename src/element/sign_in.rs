use std::{cell::RefCell, rc::Rc};

use js_sys::encode_uri_component;
use serde::Deserialize;
use yew::Callback;
use yew_router::prelude::*;

use crate::service;

pub enum Msg {
    Nothing,
    Error(String),
    SetUserId(String),
}

pub(crate) struct SignInPage {
    user_name: String,
    error: String,
}

impl yew::Component for SignInPage {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        let mut error = "".to_string();
        if let Ok(err_msg) = ctx.link().location().unwrap().query::<ErrMsg>() {
            error = err_msg.msg;
        }

        Self {
            user_name: "".to_string(),
            error,
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let user_name: Rc<RefCell<String>> = Rc::new(RefCell::new(self.user_name.clone()));
        let password: Rc<RefCell<String>> = Rc::new(RefCell::new(String::new()));

        let register = {
            let link = ctx.link().clone();
            let password = password.clone();
            yew::Callback::from(move |_| {
                link.send_future(Self::register(password.borrow().clone()))
            })
        };

        let input_user_name = {
            let link = ctx.link().clone();
            let user_name = user_name.clone();
            yew::Callback::from(move |v: String| {
                *user_name.borrow_mut() = v.clone();
                link.send_message(Msg::SetUserId(v));
            })
        };

        let input_password = {
            let password = password.clone();
            yew::Callback::from(move |v| {
                *password.borrow_mut() = v;
            })
        };

        let close = {
            let link = ctx.link().clone();
            Callback::from(move |_| link.send_message(Msg::Error(String::new())))
        };

        yew::html! {
            <div class={"page center-container"}>
                <form class={"box content"} method={"post"} action={format!("/portal/user/token?success={}&error={}", encode_uri_component("/huiwen"), encode_uri_component("/huiwen/sign_in"))}>
                    <div style={"display: flex;justify-content: center;"}>
                        <div style={"width: 6em;color: black;text-align: left;"}>{"User Name"}</div>
                        <views::Input name={"name"} value={self.user_name.clone()} update={input_user_name}></views::Input>
                    </div>
                    <br/>
                    <div style={"display: flex;justify-content: center;"}>
                        <div style={"width: 6em;color: black;text-align: left;"}>{"Password"}</div>
                        <views::Input name={"password"} r#type={"password"} update={input_password}></views::Input>
                    </div>
                    <div style={"display: flex;justify-content: space-around;margin: 2em 0 0 0;"}>
                        <views::Button onclick={register}>{"Register"}</views::Button>
                        <input type={"submit"} value={"Sign in"} />
                    </div>
                </form>
                if !self.error.is_empty() {
                    <views::Modal classes={""} {close}>
                        <div class={"content"}>{self.error.clone()}</div>
                    </views::Modal>
                }
            </div>
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Error(e) => {
                self.error = e;
                true
            }
            Msg::SetUserId(user_name) => {
                self.user_name = user_name;
                false
            }
            Msg::Nothing => false,
        }
    }
}

impl SignInPage {
    async fn register(password: String) -> Msg {
        match service::user_create("", &password).await {
            Ok(_) => Msg::Nothing,
            Err(e) => Msg::Error(e.as_string().unwrap_or("unknown error".to_string())),
        }
    }
}

#[derive(Deserialize)]
struct ErrMsg {
    msg: String,
}
