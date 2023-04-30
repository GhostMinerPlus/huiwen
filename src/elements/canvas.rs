use cgmath::*;

use std::{cell::RefCell, rc::Rc};

use painting::Frame;
use web_sys::{HtmlCanvasElement, MouseEvent, PointerEvent};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    platform::web::{EventLoopExtWebSys, WindowBuilderExtWebSys},
    window::{Window, WindowBuilder},
};

pub enum Message {
    Init(EventLoop<()>),
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
                raw_canvas.canvas.start_line(raw_canvas.pen.px2point(
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
                raw_canvas.canvas.end_line();
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
                    raw_canvas.canvas.push_point(raw_canvas.pen.px2point(
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
                raw_canvas.canvas.start_line(raw_canvas.pen.px2point(
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
                raw_canvas.canvas.end_line();
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
                    raw_canvas.canvas.push_point(raw_canvas.pen.px2point(
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
                let raw_canvas = RawCanvas::create(canvas.clone(), &event_loop).await;
                p_canvas.replace(Some(raw_canvas));
                Self::Message::Init(event_loop)
            });
        }
    }
}

struct RawCanvas {
    canvas: painting::Canvas,
    window: Window,
    pen: painting::tools::Pen,
}

impl RawCanvas {
    async fn create(n_canvas: yew::NodeRef, event_loop: &EventLoop<()>) -> Self {
        let sz = PhysicalSize::new(1024, 1024);
        let window = {
            let window = WindowBuilder::new()
                .with_canvas(n_canvas.cast())
                .build(&event_loop)
                .unwrap();
            window.set_inner_size(sz);
            window
        };
        n_canvas
            .cast::<HtmlCanvasElement>()
            .as_ref()
            .unwrap()
            .style()
            .set_css_text("");

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });
        log::info!("instance: {:?}", instance);

        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        log::info!("surface: {:?}", surface);

        let mut canvas = painting::Canvas::new(&instance, surface, window.inner_size()).await;
        canvas.start_line(painting::Point {
            pos: [0., 0., 0.].into(),
            width: 0.1,
            color: [1., 0., 0., 1.],
        });
        canvas.push_point(painting::Point {
            pos: [0., 1., 0.].into(),
            width: 0.2,
            color: [0., 1., 0., 1.],
        });
        canvas.push_point(painting::Point {
            pos: [1., 0., 0.].into(),
            width: 0.2,
            color: [0., 0., 1., 1.],
        });
        canvas.end_line();
        canvas.redraw();

        Self {
            canvas,
            window,
            pen: painting::tools::Pen::new(),
        }
    }

    fn on_event(
        &mut self,
        n_canvas: yew::NodeRef,
        event: Event<()>,
        _target: &EventLoopWindowTarget<()>,
        control_flow: &mut ControlFlow,
    ) {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == self.window.id() => match event {
                WindowEvent::Resized(physical_size) => self.canvas.resize(*physical_size),
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    self.canvas.resize(**new_inner_size)
                }
                WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                    *control_flow = ControlFlow::Exit
                }
                _ => {}
            },
            Event::RedrawRequested(window_id) if window_id == self.window.id() => {
                let sz = PhysicalSize::new(
                    n_canvas
                        .cast::<HtmlCanvasElement>()
                        .as_ref()
                        .unwrap()
                        .client_width() as u32,
                    n_canvas
                        .cast::<HtmlCanvasElement>()
                        .as_ref()
                        .unwrap()
                        .client_height() as u32,
                );
                self.canvas
                    .set_aspect((sz.width as f32) / (sz.height as f32));
                self.canvas.redraw();
            }
            _ => {}
        }
    }
}
