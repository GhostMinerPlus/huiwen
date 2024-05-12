mod raw_canvas;

use cgmath::*;
use js_sys::Math::exp;
use painting::{point::Point, AsCanvas};
use yew::{Callback, KeyboardEvent, WheelEvent};

use std::{
    io,
    sync::{Arc, Mutex},
};

use web_sys::{HtmlCanvasElement, MouseEvent, PointerEvent};
use winit::{dpi::PhysicalSize, event_loop::EventLoop, platform::web::EventLoopExtWebSys};

use crate::util::style_or;

use self::raw_canvas::RawCanvas;

// Public
pub enum Command {
    None,
    Paint,
    Move,
    Scacle,
}

pub enum Message {
    Refresh,
    Create(EventLoop<()>),
    EnableMoving,
    DisableMoving,
    StartMovingOrPainting((f32, f32, f32, f32, Option<f32>)),
    EndMovingOrPainting,
    MoveOrPaint((f32, f32, f32, f32, Option<f32>)),
    Scacle(f32),
}

#[derive(Clone, Debug, yew::Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub width: String,
    #[prop_or_default]
    pub height: String,
    #[prop_or_default]
    pub commit: Callback<Vec<Point>>,
    #[prop_or_default]
    pub edge_v: Vec<Vec<Point>>,
}

pub struct Canvas {
    canvas: yew::NodeRef,
    p_canvas: Arc<Mutex<Option<RawCanvas>>>,
    last_edge: Vec<Point>,
    enabled_moving: bool,
    cmd: Command,
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
            last_edge: Vec::default(),
            enabled_moving: false,
            cmd: Command::None,
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let link = ctx.link().clone();
        let canvas = self.canvas.cast::<HtmlCanvasElement>();
        let onmousedown = yew::Callback::from(move |e: MouseEvent| {
            let sz = PhysicalSize::new(
                canvas.as_ref().unwrap().client_width() as u32,
                canvas.as_ref().unwrap().client_height() as u32,
            );

            let x = e.offset_x() as f32;
            let y = e.offset_y() as f32;

            let v = cgmath::Vector2::new(
                (e.movement_x() as f32) / (sz.width as f32),
                (e.movement_y() as f32) / (sz.height as f32),
            );

            link.send_message(Message::StartMovingOrPainting((v.x, v.y, x, y, None)));
        });

        let link = ctx.link().clone();
        let onmouseup = yew::Callback::from(move |_| {
            link.send_message(Message::EndMovingOrPainting);
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

            let v = cgmath::Vector2::new(
                (e.movement_x() as f32) / (sz.width as f32),
                (e.movement_y() as f32) / (sz.height as f32),
            );
            link.send_message(Message::MoveOrPaint((v.x, v.y, x, y, None)));
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

            let v = cgmath::Vector2::new(
                (e.movement_x() as f32) / (sz.width as f32),
                (e.movement_y() as f32) / (sz.height as f32),
            );
            let force = if e.pointer_type() == "touch" {
                e.pressure() / 10.
            } else {
                e.pressure()
            };
            link.send_message(Message::StartMovingOrPainting((
                v.x,
                v.y,
                x,
                y,
                Some(force),
            )));
        });

        let link = ctx.link().clone();
        let onpointerup = yew::Callback::from(move |_| {
            link.send_message(Message::EndMovingOrPainting);
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

            let v = cgmath::Vector2::new(
                (e.movement_x() as f32) / (sz.width as f32),
                (e.movement_y() as f32) / (sz.height as f32),
            );

            let force = if e.pointer_type() == "touch" {
                e.pressure() / 10.
            } else {
                e.pressure()
            };
            link.send_message(Message::MoveOrPaint((v.x, v.y, x, y, Some(force))));
        });

        let link = ctx.link().clone();
        let on_key_down = Callback::from(move |e: KeyboardEvent| {
            if e.key() == " " {
                link.send_message(Message::EnableMoving);
            }
        });

        let link = ctx.link().clone();
        let on_key_up = Callback::from(move |e: KeyboardEvent| {
            if e.key() == " " {
                link.send_message(Message::DisableMoving);
            }
        });

        let link = ctx.link().clone();
        let on_wheel = Callback::from(move |e: WheelEvent| {
            let speed = e.delta_y() / 100.0;
            log::debug!("wheel speed: {speed}");
            link.send_message(Message::Scacle(exp(speed) as f32));
        });

        let style = format!(
            "{}{}",
            style_or("width", &ctx.props().width, None),
            style_or("height", &ctx.props().height, None)
        );

        yew::html! {
            <canvas ref={self.canvas.clone()}
                {style}
                {onmousedown}
                {onmouseup}
                {onmousemove}
                {onpointerdown}
                {onpointerup}
                {onpointermove}
                onkeydown={on_key_down}
                onkeyup={on_key_up}
                onwheel={on_wheel} />
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Create(event_loop) => {
                let p_canvas = self.p_canvas.clone();
                event_loop.spawn(move |event, target, control_flow| {
                    let mut op = p_canvas.lock().unwrap();
                    let raw_canvas = op.as_mut().unwrap();
                    raw_canvas.on_event(event, target, control_flow);
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
                raw_canvas.clear();

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
            Message::StartMovingOrPainting((vx, vy, x, y, force)) => {
                match &self.cmd {
                    Command::None => {
                        if self.enabled_moving {
                            log::debug!("start moving");
                            self.cmd = Command::Move;
                        } else {
                            self.cmd = Command::Paint;
                            let canvas = self.canvas.cast::<HtmlCanvasElement>();
                            let sz = PhysicalSize::new(
                                canvas.as_ref().unwrap().client_width() as u32,
                                canvas.as_ref().unwrap().client_height() as u32,
                            );
                            let mut op = self.p_canvas.lock().unwrap();
                            let raw_canvas = op.as_mut().unwrap();
                            let mut pt = match force {
                                Some(force) => raw_canvas.pen.px2point(x, y, force, sz),
                                None => {
                                    let v = cgmath::Vector2::new(vx, vy);
                                    let force = {
                                        let force = 1. - v.magnitude() * 100.;
                                        if force > 1. {
                                            1.
                                        } else if force < 0. {
                                            0.
                                        } else {
                                            force
                                        }
                                    };
                                    raw_canvas.pen.px2point(x, y, force, sz)
                                }
                            };
                            pt.pos.z = -1.0;
                            raw_canvas.start_line(pt.clone());
                            raw_canvas.window.request_redraw();
                            self.last_edge.push(pt);
                        }
                    }
                    _ => (),
                }
                false
            }
            Message::EndMovingOrPainting => {
                match &self.cmd {
                    Command::Paint => {
                        if self.last_edge.is_empty() {
                            return false;
                        }
                        let mut op = self.p_canvas.lock().unwrap();
                        let raw_canvas = op.as_mut().unwrap();
                        raw_canvas.end_line();
                        raw_canvas.window.request_redraw();
                        ctx.props().commit.emit(self.last_edge.clone());
                        self.last_edge.clear();
                    }
                    Command::Move => {
                        log::debug!("end moving");
                    }
                    _ => (),
                }
                self.cmd = Command::None;
                false
            }
            Message::EnableMoving => {
                self.enabled_moving = true;
                false
            }
            Message::DisableMoving => {
                self.enabled_moving = false;
                false
            }
            Message::MoveOrPaint((vx, vy, x, y, force)) => {
                match &self.cmd {
                    Command::Move => {
                        log::debug!("moving");
                        let mut op = self.p_canvas.lock().unwrap();
                        let raw_canvas = op.as_mut().unwrap();
                        raw_canvas.move_content(vx, -vy, 0.0);
                        raw_canvas.window.request_redraw();
                    }
                    Command::Paint => {
                        let force = match force {
                            Some(force) => force,
                            None => {
                                let v = cgmath::Vector2::new(vx, vy);

                                let force = 1. - v.magnitude() * 100.;
                                if force > 1. {
                                    1.
                                } else if force < 0. {
                                    0.
                                } else {
                                    force
                                }
                            }
                        };
                        let mut op = self.p_canvas.lock().unwrap();
                        let raw_canvas = op.as_mut().unwrap();
                        let canvas = self.canvas.cast::<HtmlCanvasElement>();
                        let sz = PhysicalSize::new(
                            canvas.as_ref().unwrap().client_width() as u32,
                            canvas.as_ref().unwrap().client_height() as u32,
                        );
                        let mut pt = raw_canvas.pen.px2point(x, y, force, sz);
                        pt.pos.z = -1.0;

                        raw_canvas.push_point(pt.clone());
                        raw_canvas.window.request_redraw();
                        self.last_edge.push(pt);
                    }
                    _ => (),
                }
                false
            }
            Message::Scacle(s) => {
                match &self.cmd {
                    Command::None | Command::Scacle => {
                        self.cmd = Command::Scacle;

                        let mut op = self.p_canvas.lock().unwrap();
                        let raw_canvas = op.as_mut().unwrap();
                        raw_canvas.scacle(s, s, 1.0);
                        raw_canvas.window.request_redraw();

                        self.cmd = Command::None;
                    }
                    _ => (),
                }
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
                let html_canvas = canvas
                    .cast::<HtmlCanvasElement>()
                    // .as_ref()
                    .ok_or(io::Error::new(
                        io::ErrorKind::NotFound,
                        "'HtmlCanvasElement' not found",
                    ))?;
                let raw_canvas = RawCanvas::create(html_canvas, &event_loop).await?;
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
