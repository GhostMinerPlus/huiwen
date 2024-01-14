mod raw_canvas;

use cgmath::*;
use painting::{point::Point, AsPainter};
use yew::{classes, Callback};

use std::{
    io,
    sync::{Arc, Mutex},
};

use web_sys::{HtmlCanvasElement, MouseEvent, PointerEvent};
use winit::{dpi::PhysicalSize, event_loop::EventLoop, platform::web::EventLoopExtWebSys};

use self::raw_canvas::RawCanvas;

pub enum Message {
    Refresh,
    Create(EventLoop<()>),
    StartPainting((f32, f32, f32, PhysicalSize<u32>)),
    Paint((f32, f32, f32, PhysicalSize<u32>)),
    EndLine,
}

#[derive(Clone, Debug, yew::Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub classes: yew::Classes,
    #[prop_or_default]
    pub commit: Callback<Vec<Point>>,
    #[prop_or_default]
    pub edge_v: Vec<Vec<Point>>,
}

pub struct Canvas {
    canvas: yew::NodeRef,
    p_canvas: Arc<Mutex<Option<RawCanvas>>>,
    painting: bool,
    last_edge: Vec<Point>,
}

impl yew::Component for Canvas {
    type Message = Message;

    type Properties = Props;

    fn create(_: &yew::Context<Self>) -> Self {
        let canvas = yew::NodeRef::default();
        let p_canvas = Arc::new(Mutex::new(None));

        Self {
            canvas,
            p_canvas,
            painting: false,
            last_edge: Vec::default(),
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

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Create(event_loop) => {
                let canvas = self.canvas.clone();
                let p_canvas = self.p_canvas.clone();
                event_loop.spawn(move |event, target, control_flow| {
                    let mut op = p_canvas.lock().unwrap();
                    let raw_canvas = op.as_mut().unwrap();
                    raw_canvas.on_event(canvas.clone(), event, target, control_flow);
                });
                ctx.link().send_message(Message::Refresh);
                true
            }
            Message::Refresh => {
                let mut op = self.p_canvas.lock().unwrap();
                if op.is_none() {
                    return false;
                }
                let raw_canvas = op.as_mut().unwrap();

                let edge_v = &ctx.props().edge_v;
                for edge in edge_v {
                    raw_canvas.start_line(edge[0].clone());
                    for i in 1..edge.len() {
                        raw_canvas.push_point(edge[i].clone());
                    }
                    raw_canvas.end_line();
                }
                raw_canvas.window.request_redraw();
                false
            }
            Message::StartPainting((x, y, force, sz)) => {
                self.painting = true;
                let mut op = self.p_canvas.lock().unwrap();
                let raw_canvas = op.as_mut().unwrap();
                let pt = raw_canvas.pen.px2point(x, y, force, sz);
                raw_canvas.start_line(pt.clone());
                raw_canvas.window.request_redraw();
                self.last_edge.push(pt);
                false
            }
            Message::Paint((x, y, force, sz)) => {
                if self.painting {
                    let mut op = self.p_canvas.lock().unwrap();
                    let raw_canvas = op.as_mut().unwrap();
                    let pt = raw_canvas.pen.px2point(x, y, force, sz);
                    raw_canvas.push_point(pt.clone());
                    raw_canvas.window.request_redraw();
                    self.last_edge.push(pt);
                }
                false
            }
            Message::EndLine => {
                self.painting = false;
                if self.last_edge.is_empty() {
                    return false;
                }
                let mut op = self.p_canvas.lock().unwrap();
                let raw_canvas = op.as_mut().unwrap();
                raw_canvas.end_line();
                raw_canvas.window.request_redraw();
                ctx.props().commit.emit(self.last_edge.clone());
                self.last_edge.clear();
                false
            }
        }
    }

    fn changed(
        &mut self,
        ctx: &yew::prelude::Context<Self>,
        _old_props: &Self::Properties,
    ) -> bool {
        ctx.link().send_message(Message::Refresh);
        false
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
            let rs: io::Result<Message> = async {
                let event_loop = EventLoop::new();
                let raw_canvas = RawCanvas::create(canvas.clone(), &event_loop).await?;
                let mut p_canvas = p_canvas.lock().unwrap();
                *p_canvas = Some(raw_canvas);
                Ok(Message::Create(event_loop))
            }
            .await;
            match rs {
                Ok(msg) => msg,
                Err(e) => panic!("Failed to build raw_canvas: {:?}", e),
            }
        });
    }
}
