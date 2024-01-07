use painting::Frame;
use web_sys::HtmlCanvasElement;
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    platform::web::WindowBuilderExtWebSys,
    window::{Window, WindowBuilder},
};

pub struct RawCanvas {
    canvas: painting::Canvas,
    pub window: Window,
    pub pen: painting::tools::Pen,
}

impl RawCanvas {
    pub async fn create(
        n_canvas: yew::NodeRef,
        event_loop: &EventLoop<()>,
    ) -> Result<Self, String> {
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

        let mut canvas = painting::Canvas::create(&instance, surface, window.inner_size()).await?;

        let line_v = moon::execute(json::array![["watch", "line"]]).unwrap();
        for i in 0..line_v.len() {
            let line = &line_v[i.to_string().as_str()];
            let pt = &line["0"];
            canvas.start_line(painting::Point {
                pos: cgmath::Point3 {
                    x: pt["pos"]["x"].as_str().unwrap().parse().unwrap(),
                    y: pt["pos"]["y"].as_str().unwrap().parse().unwrap(),
                    z: pt["pos"]["z"].as_str().unwrap().parse().unwrap(),
                },
                color: [
                    pt["color"]["r"].as_str().unwrap().parse().unwrap(),
                    pt["color"]["g"].as_str().unwrap().parse().unwrap(),
                    pt["color"]["b"].as_str().unwrap().parse().unwrap(),
                    pt["color"]["a"].as_str().unwrap().parse().unwrap(),
                ],
                width: pt["width"].as_str().unwrap().parse().unwrap(),
            });
            for j in 1..line.len() {
                let pt = &line[j.to_string().as_str()];
                canvas.push_point(painting::Point {
                    pos: cgmath::Point3 {
                        x: pt["pos"]["x"].as_str().unwrap().parse().unwrap(),
                        y: pt["pos"]["y"].as_str().unwrap().parse().unwrap(),
                        z: pt["pos"]["z"].as_str().unwrap().parse().unwrap(),
                    },
                    color: [
                        pt["color"]["r"].as_str().unwrap().parse().unwrap(),
                        pt["color"]["g"].as_str().unwrap().parse().unwrap(),
                        pt["color"]["b"].as_str().unwrap().parse().unwrap(),
                        pt["color"]["a"].as_str().unwrap().parse().unwrap(),
                    ],
                    width: pt["width"].as_str().unwrap().parse().unwrap(),
                });
            }
            canvas.end_line();
        }

        Ok(Self {
            canvas,
            window,
            pen: painting::tools::Pen::new(),
        })
    }

    pub fn on_event(
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
                self.redraw();
            }
            _ => {}
        }
    }
}

impl painting::Frame for RawCanvas {
    fn redraw(&mut self) {
        let _ = moon::execute(json::array![["redraw"]]);
        self.canvas.redraw();
    }

    fn push_point(&mut self, pt: painting::Point) {
        let _ = moon::execute(json::array![["push_point", {
            "width": format!("{}", pt.width),
            "pos": {
                "x": format!("{}", pt.pos.x),
                "y": format!("{}", pt.pos.y),
                "z": format!("{}", pt.pos.z)
            },
            "color": {
                "r": format!("{}", pt.color[0]),
                "g": format!("{}", pt.color[1]),
                "b": format!("{}", pt.color[2]),
                "a": format!("{}", pt.color[3])
            }
        }]]);
        self.canvas.push_point(pt);
    }

    fn start_line(&mut self, pt: painting::Point) {
        let _ = moon::execute(json::array![["start_line", {
            "width": format!("{}", pt.width),
            "pos": {
                "x": format!("{}", pt.pos.x),
                "y": format!("{}", pt.pos.y),
                "z": format!("{}", pt.pos.z)
            },
            "color": {
                "r": format!("{}", pt.color[0]),
                "g": format!("{}", pt.color[1]),
                "b": format!("{}", pt.color[2]),
                "a": format!("{}", pt.color[3])
            }
        }]]);
        self.canvas.start_line(pt);
    }

    fn end_line(&mut self) {
        let _ = moon::execute(json::array![["end_line"]]);
        self.canvas.end_line();
    }

    fn cancle_line(&mut self) {
        let _ = moon::execute(json::array![["cancle_line"]]);
        self.canvas.cancle_line();
    }

    fn set_aspect(&mut self, aspect: f32) {
        let _ = moon::execute(json::array![["set_aspect", format!("{aspect}")]]);
        self.canvas.set_aspect(aspect);
    }
}
