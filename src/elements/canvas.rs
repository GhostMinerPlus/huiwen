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

#[derive(Clone, Debug, yew::Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub classes: yew::Classes,
}

pub struct Canvas {
    canvas: yew::NodeRef,
    p_canvas: Rc<RefCell<Option<painting::Canvas>>>,
    window: Rc<RefCell<Option<Window>>>,
    pen: painting::tools::Pen,
}

impl yew::Component for Canvas {
    type Message = ();

    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let canvas = yew::NodeRef::default();
        let p_canvas = Rc::new(RefCell::new(None));
        let window = Rc::new(RefCell::new(None));
        ctx.link().send_future({
            let canvas = canvas.clone();
            let p_canvas = p_canvas.clone();
            let window = window.clone();
            Self::init(canvas, window, p_canvas)
        });
        Self {
            canvas,
            p_canvas,
            window,
            pen: painting::tools::Pen::new(),
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        static mut PAINTING: bool = false;

        let props = ctx.props();

        let onmousedown = {
            let canvas = self.canvas.clone();
            let p_canvas = self.p_canvas.clone();
            let window = self.window.clone();
            let pen = self.pen.clone();
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

                let mut binding = p_canvas.borrow_mut();
                let canvas = binding.as_mut().unwrap();
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
                canvas.start_line(pen.px2point(
                    e.offset_x() as f32,
                    e.offset_y() as f32,
                    force,
                    sz,
                ));
                let mut window_binding = window.borrow_mut();
                let window = window_binding.as_mut().unwrap();
                window.request_redraw();
            })
        };

        let onmouseup = {
            let p_canvas = self.p_canvas.clone();
            let window = self.window.clone();
            yew::Callback::from(move |_| unsafe {
                PAINTING = false;

                let mut binding = p_canvas.borrow_mut();
                let canvas = binding.as_mut().unwrap();
                canvas.end_line();
                let mut window_binding = window.borrow_mut();
                let window = window_binding.as_mut().unwrap();
                window.request_redraw();
            })
        };

        let onmousemove = {
            let canvas = self.canvas.clone();
            let p_canvas = self.p_canvas.clone();
            let window = self.window.clone();
            let pen = self.pen.clone();
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

                    let mut binding = p_canvas.borrow_mut();
                    let canvas = binding.as_mut().unwrap();
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
                    canvas.push_point(pen.px2point(
                        e.offset_x() as f32,
                        e.offset_y() as f32,
                        force,
                        sz,
                    ));
                    let mut window_binding = window.borrow_mut();
                    let window = window_binding.as_mut().unwrap();
                    window.request_redraw();
                }
            })
        };

        let onpointerdown = {
            let canvas = self.canvas.clone();
            let p_canvas = self.p_canvas.clone();
            let window = self.window.clone();
            let pen = self.pen.clone();
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
                let canvas = binding.as_mut().unwrap();
                canvas.start_line(pen.px2point(
                    e.offset_x() as f32,
                    e.offset_y() as f32,
                    if e.pointer_type() == "touch" {
                        e.pressure() / 10.
                    } else {
                        e.pressure()
                    },
                    sz,
                ));
                let mut window_binding = window.borrow_mut();
                let window = window_binding.as_mut().unwrap();
                window.request_redraw();
            })
        };

        let onpointerup = {
            let p_canvas = self.p_canvas.clone();
            let window = self.window.clone();
            yew::Callback::from(move |_| unsafe {
                PAINTING = false;

                let mut binding = p_canvas.borrow_mut();
                let canvas = binding.as_mut().unwrap();
                canvas.end_line();
                let mut window_binding = window.borrow_mut();
                let window = window_binding.as_mut().unwrap();
                window.request_redraw();
            })
        };

        let onpointermove = {
            let canvas = self.canvas.clone();
            let p_canvas = self.p_canvas.clone();
            let window = self.window.clone();
            let pen = self.pen.clone();
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
                    let canvas = binding.as_mut().unwrap();
                    canvas.push_point(pen.px2point(
                        e.offset_x() as f32,
                        e.offset_y() as f32,
                        if e.pointer_type() == "touch" {
                            e.pressure() / 10.
                        } else {
                            e.pressure()
                        },
                        sz,
                    ));
                    let mut window_binding = window.borrow_mut();
                    let window = window_binding.as_mut().unwrap();
                    window.request_redraw();
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
}

impl Canvas {
    async fn init(
        n_canvas: yew::NodeRef,
        p_window: Rc<RefCell<Option<Window>>>,
        p_canvas: Rc<RefCell<Option<painting::Canvas>>>,
    ) {
        let sz = PhysicalSize::new(1024, 1024);
        let event_loop = EventLoop::new();
        let window = {
            let window = WindowBuilder::new()
                .with_canvas(n_canvas.cast())
                .build(&event_loop)
                .unwrap();
            window.set_inner_size(sz);
            window
        };

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });
        let surface = unsafe { instance.create_surface(&window) }.unwrap();
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

        n_canvas
            .cast::<HtmlCanvasElement>()
            .as_ref()
            .unwrap()
            .style()
            .set_css_text("");
        canvas.redraw();
        p_canvas.replace(Some(canvas));
        p_window.replace(Some(window));
        event_loop.spawn(move |event, target, control_flow| {
            Self::on_event(
                n_canvas.clone(),
                p_window.clone(),
                p_canvas.clone(),
                event,
                target,
                control_flow,
            );
        });
    }

    fn on_event(
        n_canvas: yew::NodeRef,
        window: Rc<RefCell<Option<Window>>>,
        p_canvas: Rc<RefCell<Option<painting::Canvas>>>,
        event: Event<()>,
        _: &EventLoopWindowTarget<()>,
        control_flow: &mut ControlFlow,
    ) {
        let mut window_binding = window.borrow_mut();
        let window = window_binding.as_mut().unwrap();
        let mut binding = p_canvas.borrow_mut();
        let canvas = binding.as_mut().unwrap();
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::Resized(physical_size) => canvas.resize(*physical_size),
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    canvas.resize(**new_inner_size)
                }
                WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                    *control_flow = ControlFlow::Exit
                }
                _ => {}
            },
            Event::RedrawRequested(window_id) if window_id == window.id() => {
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
                canvas.set_aspect((sz.width as f32) / (sz.height as f32));
                canvas.redraw();
            }
            _ => {}
        }
    }
}
