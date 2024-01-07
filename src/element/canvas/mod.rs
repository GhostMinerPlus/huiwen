mod raw_canvas;

use cgmath::*;
use painting::Frame;

use std::{cell::RefCell, rc::Rc};

use web_sys::{HtmlCanvasElement, MouseEvent, PointerEvent};
use winit::{dpi::PhysicalSize, event_loop::EventLoop, platform::web::EventLoopExtWebSys};

use self::raw_canvas::RawCanvas;

pub enum Message {
    Init(EventLoop<()>),
    None,
}

#[derive(Clone, Debug, yew::Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub classes: yew::Classes,
}

pub struct Canvas {
    canvas: yew::NodeRef,
    p_canvas: Rc<RefCell<Option<RawCanvas>>>,
}

impl yew::Component for Canvas {
    type Message = Message;

    type Properties = Props;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        let canvas = yew::NodeRef::default();
        let p_canvas = Rc::new(RefCell::new(None));

        Self { canvas, p_canvas }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        static mut PAINTING: bool = false;

        let props = ctx.props();

        let onmousedown = {
            let canvas = self.canvas.clone();
            let p_canvas = self.p_canvas.clone();
            yew::Callback::from(move |e: MouseEvent| unsafe {
                PAINTING = true;

                let sz = PhysicalSize::new(
                    canvas
                        .cast::<HtmlCanvasElement>()
                        .as_ref()
                        .unwrap()
                        .client_width() as u32,
                    canvas
                        .cast::<HtmlCanvasElement>()
                        .as_ref()
                        .unwrap()
                        .client_height() as u32,
                );

                let force = {
                    let v = cgmath::Vector2::new(
                        (e.movement_x() as f32) / (sz.width as f32),
                        (e.movement_y() as f32) / (sz.height as f32),
                    );
                    let force = 1. - v.magnitude() * 100.;
                    if force > 1. {
                        1.
                    } else if force < 0. {
                        0.
                    } else {
                        force
                    }
                };
                let mut binding = p_canvas.borrow_mut();
                let raw_canvas = binding.as_mut().unwrap();
                raw_canvas.start_line(raw_canvas.pen.px2point(
                    e.offset_x() as f32,
                    e.offset_y() as f32,
                    force,
                    sz,
                ));
                raw_canvas.window.request_redraw();
            })
        };

        let onmouseup = {
            let p_canvas = self.p_canvas.clone();
            yew::Callback::from(move |_| unsafe {
                PAINTING = false;

                let mut binding = p_canvas.borrow_mut();
                let raw_canvas = binding.as_mut().unwrap();
                raw_canvas.end_line();
                raw_canvas.window.request_redraw();
            })
        };

        let onmousemove = {
            let canvas = self.canvas.clone();
            let p_canvas = self.p_canvas.clone();
            yew::Callback::from(move |e: MouseEvent| unsafe {
                if PAINTING {
                    let sz = PhysicalSize::new(
                        canvas
                            .cast::<HtmlCanvasElement>()
                            .as_ref()
                            .unwrap()
                            .client_width() as u32,
                        canvas
                            .cast::<HtmlCanvasElement>()
                            .as_ref()
                            .unwrap()
                            .client_height() as u32,
                    );

                    let force = {
                        let v = cgmath::Vector2::new(
                            (e.movement_x() as f32) / (sz.width as f32),
                            (e.movement_y() as f32) / (sz.height as f32),
                        );
                        let force = 1. - v.magnitude() * 100.;
                        if force > 1. {
                            1.
                        } else if force < 0. {
                            0.
                        } else {
                            force
                        }
                    };
                    let mut binding = p_canvas.borrow_mut();
                    let raw_canvas = binding.as_mut().unwrap();
                    raw_canvas.push_point(raw_canvas.pen.px2point(
                        e.offset_x() as f32,
                        e.offset_y() as f32,
                        force,
                        sz,
                    ));
                    raw_canvas.window.request_redraw();
                }
            })
        };

        let onpointerdown = {
            let canvas = self.canvas.clone();
            let p_canvas = self.p_canvas.clone();
            yew::Callback::from(move |e: PointerEvent| unsafe {
                PAINTING = true;

                let sz = PhysicalSize::new(
                    canvas
                        .cast::<HtmlCanvasElement>()
                        .as_ref()
                        .unwrap()
                        .client_width() as u32,
                    canvas
                        .cast::<HtmlCanvasElement>()
                        .as_ref()
                        .unwrap()
                        .client_height() as u32,
                );

                let mut binding = p_canvas.borrow_mut();
                let raw_canvas = binding.as_mut().unwrap();
                raw_canvas.start_line(raw_canvas.pen.px2point(
                    e.offset_x() as f32,
                    e.offset_y() as f32,
                    if e.pointer_type() == "touch" {
                        e.pressure() / 10.
                    } else {
                        e.pressure()
                    },
                    sz,
                ));
                raw_canvas.window.request_redraw();
            })
        };

        let onpointerup = {
            let p_canvas = self.p_canvas.clone();
            yew::Callback::from(move |_| unsafe {
                PAINTING = false;

                let mut binding = p_canvas.borrow_mut();
                let raw_canvas = binding.as_mut().unwrap();
                raw_canvas.end_line();
                raw_canvas.window.request_redraw();
            })
        };

        let onpointermove = {
            let canvas = self.canvas.clone();
            let p_canvas = self.p_canvas.clone();
            yew::Callback::from(move |e: PointerEvent| unsafe {
                if PAINTING {
                    let sz = PhysicalSize::new(
                        canvas
                            .cast::<HtmlCanvasElement>()
                            .as_ref()
                            .unwrap()
                            .client_width() as u32,
                        canvas
                            .cast::<HtmlCanvasElement>()
                            .as_ref()
                            .unwrap()
                            .client_height() as u32,
                    );

                    let mut binding = p_canvas.borrow_mut();
                    let raw_canvas = binding.as_mut().unwrap();
                    raw_canvas.push_point(raw_canvas.pen.px2point(
                        e.offset_x() as f32,
                        e.offset_y() as f32,
                        if e.pointer_type() == "touch" {
                            e.pressure() / 10.
                        } else {
                            e.pressure()
                        },
                        sz,
                    ));
                    raw_canvas.window.request_redraw();
                }
            })
        };

        yew::html! {
            <canvas ref={self.canvas.clone()} class={props.classes.clone()}
                {onmousedown}
                {onmouseup}
                {onmousemove}
                {onpointerdown}
                {onpointerup}
                {onpointermove}>
            </canvas>
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Init(event_loop) => {
                let canvas = self.canvas.clone();
                let p_canvas = self.p_canvas.clone();
                event_loop.spawn(move |event, target, control_flow| {
                    let mut binding = p_canvas.borrow_mut();
                    let raw_canvas = binding.as_mut().unwrap();
                    raw_canvas.on_event(canvas.clone(), event, target, control_flow);
                });
                true
            }
            Message::None => false,
        }
    }

    fn rendered(&mut self, ctx: &yew::Context<Self>, first_render: bool) {
        if first_render {
            if self.p_canvas.borrow().is_some() {
                return;
            }

            let canvas = self.canvas.clone();
            let p_canvas = self.p_canvas.clone();
            ctx.link().send_future(async move {
                let event_loop = EventLoop::new();
                let raw_canvas = match RawCanvas::create(canvas.clone(), &event_loop).await {
                    Ok(o) => o,
                    Err(e) => {
                        log::error!("failed to build raw_canvas: {:?}", e);
                        return Self::Message::None;
                    }
                };
                p_canvas.replace(Some(raw_canvas));
                Self::Message::Init(event_loop)
            });
        }
    }
}
