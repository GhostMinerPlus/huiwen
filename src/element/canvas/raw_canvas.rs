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

        let canvas = painting::Canvas::create(&instance, surface, window.inner_size()).await?;

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
        self.canvas.redraw();
    }

    fn push_point(&mut self, pt: painting::Point) {
        self.canvas.push_point(pt);
    }

    fn start_line(&mut self, pt: painting::Point) {
        self.canvas.start_line(pt);
    }

    fn end_line(&mut self) {
        self.canvas.end_line();
    }

    fn cancle_line(&mut self) {
        self.canvas.cancle_line();
    }

    fn set_aspect(&mut self, aspect: f32) {
        self.canvas.set_aspect(aspect);
    }
}
