use std::{cell::RefCell, rc::Rc};

use yew_router::prelude::*;

use crate::app::services;

pub enum Msg {
    Success,
    Error(String),
    SetUserId(String),
}

pub(crate) struct SignInPage {
    user_id: String,
}

impl yew::Component for SignInPage {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            user_id: String::new(),
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let user_id: Rc<RefCell<String>> = Rc::new(RefCell::new(self.user_id.clone()));
        let password: Rc<RefCell<String>> = Rc::new(RefCell::new(String::new()));

        let register = {
            let link = ctx.link().clone();
            let password = password.clone();
            yew::Callback::from(move |_| {
                link.send_future(Self::register(password.borrow().clone()))
            })
        };

        let sign_in = {
            let link = ctx.link().clone();
            let user_id = user_id.clone();
            let password = password.clone();
            yew::Callback::from(move |_| {
                link.send_future(Self::sign_in(
                    user_id.borrow().clone(),
                    password.borrow().clone(),
                ));
            })
        };

        let input_user_id = {
            let link = ctx.link().clone();
            let user_id = user_id.clone();
            yew::Callback::from(move |v: String| {
                *user_id.borrow_mut() = v.clone();
                link.send_message(Msg::SetUserId(v));
            })
        };

        let input_password = {
            let password = password.clone();
            yew::Callback::from(move |v| {
                *password.borrow_mut() = v;
            })
        };

        yew::html! {
            <div class={"page center-container"}>
                <div style={"width: 62%;background: white;padding: 3em;"}>
                    <div style={"display: flex;justify-content: center;"}>
                        <div style={"width: 6em;color: black;text-align: left;"}>{"User ID"}</div>
                        <huiwen::Input value={self.user_id.clone()} update={input_user_id}></huiwen::Input>
                    </div>
                    <br/>
                    <div style={"display: flex;justify-content: center;"}>
                        <div style={"width: 6em;color: black;text-align: left;"}>{"Password"}</div>
                        <huiwen::Input update={input_password}></huiwen::Input>
                    </div>
                    <div style={"display: flex;justify-content: space-around;margin: 2em 0 0 0;"}>
                        <huiwen::Button onclick={register}>{"Register"}</huiwen::Button>
                        <huiwen::Button onclick={sign_in}>{"Sign in"}</huiwen::Button>
                    </div>
                </div>
            </div>
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Success => {
                ctx.link().navigator().unwrap().back();
                false
            }
            Msg::Error(e) => {
                log::info!("{e}");
                true
            }
            Msg::SetUserId(user_id) => {
                self.user_id = user_id;
                true
            }
        }
    }
}

impl SignInPage {
    async fn register(password: String) -> Msg {
        match services::create_user("", &password).await {
            Ok(o) => Self::sign_in(o, password).await,
            Err(e) => Msg::Error(e.as_string().unwrap_or("unknown error".to_string())),
        }
    }

    async fn sign_in(user_id: String, password: String) -> Msg {
        match services::create_token(&user_id, &password).await {
            Ok(_) => Msg::Success,
            Err(e) => Msg::Error(e.as_string().unwrap_or("unknown error".to_string())),
        }
    }
}
