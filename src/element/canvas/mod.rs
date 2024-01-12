mod raw_canvas;

use cgmath::*;
use painting::AsPainter;
use yew::classes;

use std::sync::{Arc, Mutex};

use web_sys::{HtmlCanvasElement, MouseEvent, PointerEvent};
use winit::{dpi::PhysicalSize, event_loop::EventLoop, platform::web::EventLoopExtWebSys};

use self::raw_canvas::RawCanvas;

pub enum Message {
    Init(EventLoop<()>),
    StartPainting((f32, f32, f32, PhysicalSize<u32>)),
    Paint((f32, f32, f32, PhysicalSize<u32>)),
    EndLine,
    Nothing,
}

#[derive(Clone, Debug, yew::Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub classes: yew::Classes,
}

pub struct Canvas {
    canvas: yew::NodeRef,
    p_canvas: Arc<Mutex<Option<RawCanvas>>>,
    painting: bool,
}

impl yew::Component for Canvas {
    type Message = Message;

    type Properties = Props;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        let canvas = yew::NodeRef::default();
        let p_canvas = Arc::new(Mutex::new(None));

        Self {
            canvas,
            p_canvas,
            painting: false,
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let props = ctx.props();

        let link = ctx.link().clone();
        let canvas = self.canvas.cast::<HtmlCanvasElement>();
        let onmousedown = yew::Callback::from(move |e: MouseEvent| {
            let sz = PhysicalSize::new(
                canvas.as_ref().unwrap().client_width() as u32,
                canvas.as_ref().unwrap().client_height() as u32,
            );

            let x = e.offset_x() as f32;
            let y = e.offset_y() as f32;

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

            link.send_message(Message::StartPainting((x, y, force, sz)));
        });

        let link = ctx.link().clone();
        let onmouseup = yew::Callback::from(move |_| {
            link.send_message(Message::EndLine);
        });

        let link = ctx.link().clone();
        let canvas = self.canvas.cast::<HtmlCanvasElement>();
        let onmousemove = yew::Callback::from(move |e: MouseEvent| {
            let sz = PhysicalSize::new(
                canvas.as_ref().unwrap().client_width() as u32,
                canvas.as_ref().unwrap().client_height() as u32,
            );

            let x = e.offset_x() as f32;
            let y = e.offset_y() as f32;

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
            link.send_message(Message::Paint((x, y, force, sz)));
        });

        let link = ctx.link().clone();
        let canvas = self.canvas.cast::<HtmlCanvasElement>();
        let onpointerdown = yew::Callback::from(move |e: PointerEvent| {
            let sz = PhysicalSize::new(
                canvas.as_ref().unwrap().client_width() as u32,
                canvas.as_ref().unwrap().client_height() as u32,
            );

            let x = e.offset_x() as f32;
            let y = e.offset_y() as f32;

            let force = if e.pointer_type() == "touch" {
                e.pressure() / 10.
            } else {
                e.pressure()
            };
            link.send_message(Message::StartPainting((x, y, force, sz)));
        });

        let link = ctx.link().clone();
        let onpointerup = yew::Callback::from(move |_| {
            link.send_message(Message::EndLine);
        });

        let link = ctx.link().clone();
        let canvas = self.canvas.cast::<HtmlCanvasElement>();
        let onpointermove = yew::Callback::from(move |e: PointerEvent| {
            let sz = PhysicalSize::new(
                canvas.as_ref().unwrap().client_width() as u32,
                canvas.as_ref().unwrap().client_height() as u32,
            );

            let x = e.offset_x() as f32;
            let y = e.offset_y() as f32;

            let force = if e.pointer_type() == "touch" {
                e.pressure() / 10.
            } else {
                e.pressure()
            };
            link.send_message(Message::Paint((x, y, force, sz)));
        });

        yew::html! {
            <div class={"content"}>
                <canvas ref={self.canvas.clone()} class={classes!("content", props.classes.clone())}
                    {onmousedown}
                    {onmouseup}
                    {onmousemove}
                    {onpointerdown}
                    {onpointerup}
                    {onpointermove}>
                </canvas>
            </div>
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Init(event_loop) => {
                let canvas = self.canvas.clone();
                let p_canvas = self.p_canvas.clone();
                event_loop.spawn(move |event, target, control_flow| {
                    let mut op = p_canvas.lock().unwrap();
                    let raw_canvas = op.as_mut().unwrap();
                    raw_canvas.on_event(canvas.clone(), event, target, control_flow);
                });
                true
            }
            Message::Nothing => false,
            Message::StartPainting((x, y, force, sz)) => {
                self.painting = true;
                let mut op = self.p_canvas.lock().unwrap();
                let raw_canvas = op.as_mut().unwrap();
                raw_canvas.start_line(raw_canvas.pen.px2point(x, y, force, sz));
                raw_canvas.window.request_redraw();
                false
            }
            Message::EndLine => {
                self.painting = false;
                let mut op = self.p_canvas.lock().unwrap();
                let raw_canvas = op.as_mut().unwrap();
                raw_canvas.end_line();
                raw_canvas.window.request_redraw();
                false
            }
            Message::Paint((x, y, force, sz)) => {
                if self.painting {
                    let mut op = self.p_canvas.lock().unwrap();
                    let raw_canvas = op.as_mut().unwrap();
                    raw_canvas.push_point(raw_canvas.pen.px2point(x, y, force, sz));
                    raw_canvas.window.request_redraw();
                }
                false
            }
        }
    }

    fn rendered(&mut self, ctx: &yew::Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }
        let p_canvas = self.p_canvas.lock().unwrap();
        if p_canvas.is_some() {
            return;
        }
        drop(p_canvas);

        let canvas = self.canvas.clone();
        let p_canvas = self.p_canvas.clone();
        ctx.link().send_future(async move {
            let event_loop = EventLoop::new();
            let raw_canvas = match RawCanvas::create(canvas.clone(), &event_loop).await {
                Ok(o) => o,
                Err(e) => {
                    log::error!("failed to build raw_canvas: {:?}", e);
                    return Message::Nothing;
                }
            };
            let mut p_canvas = p_canvas.lock().unwrap();
            *p_canvas = Some(raw_canvas);
            Message::Init(event_loop)
        });
    }
}
