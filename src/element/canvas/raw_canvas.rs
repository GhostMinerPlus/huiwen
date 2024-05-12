use std::io;

use painting::AsCanvas;
use web_sys::HtmlCanvasElement;
use wgpu::SurfaceError;
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    platform::web::WindowBuilderExtWebSys,
    window::{Window, WindowBuilder},
};

// Public
pub struct RawCanvas {
    canvas: painting::Canvas,
    html_canvas: HtmlCanvasElement,
    pub window: Window,
    pub pen: painting::point::Pen,
}

impl RawCanvas {
    pub async fn create(
        html_canvas: HtmlCanvasElement,
        event_loop: &EventLoop<()>,
    ) -> io::Result<Self> {
        let sz = PhysicalSize::new(
            html_canvas.client_width() as u32,
            html_canvas.client_height() as u32,
        );
        let window = WindowBuilder::new()
            .with_canvas(Some(html_canvas.clone()))
            .build(&event_loop)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        window.set_inner_size(sz);

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });
        log::info!("instance: {:?}", instance);

        let surface = unsafe { instance.create_surface(&window) }
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        log::info!("surface: {:?}", surface);

        let canvas = painting::Canvas::create(&instance, surface, window.inner_size()).await?;

        Ok(Self {
            canvas,
            html_canvas,
            window,
            pen: painting::point::Pen::default(),
        })
    }

    pub fn on_event(
        &mut self,
        event: Event<()>,
        _target: &EventLoopWindowTarget<()>,
        control_flow: &mut ControlFlow,
    ) {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == self.window.id() => match event {
                WindowEvent::Resized(physical_size) => {
                    self.resize(*physical_size);
                    let _ = self.render();
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    self.resize(**new_inner_size);
                    let _ = self.render();
                }
                WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                    *control_flow = ControlFlow::Exit
                }
                _ => {}
            },
            Event::RedrawRequested(window_id) if window_id == self.window.id() => {
                let sz = PhysicalSize::new(
                    self.html_canvas.client_width() as u32,
                    self.html_canvas.client_height() as u32,
                );
                self.canvas
                    .set_aspect((sz.width as f32) / (sz.height as f32));
                let _ = self.render();
            }
            _ => {}
        }
    }
}

impl painting::AsCanvas for RawCanvas {
    fn render(&mut self) -> Result<(), SurfaceError> {
        self.canvas.render()
    }

    fn push_point(&mut self, pt: painting::point::Point) {
        self.canvas.push_point(pt)
    }

    fn start_line(&mut self, pt: painting::point::Point) {
        self.canvas.start_line(pt)
    }

    fn end_line(&mut self) {
        self.canvas.end_line()
    }

    fn cancle_line(&mut self) {
        self.canvas.cancle_line()
    }

    fn set_aspect(&mut self, aspect: f32) {
        self.canvas.set_aspect(aspect)
    }

    fn clear(&mut self) {
        self.canvas.clear()
    }

    fn get_size(&self) -> &PhysicalSize<u32> {
        self.canvas.get_size()
    }

    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.canvas.resize(new_size)
    }

    fn move_content(&mut self, x: f32, y: f32, z: f32) {
        self.canvas.move_content(x, y, z)
    }

    fn scacle(&mut self, x: f32, y: f32, z: f32) {
        self.canvas.scacle(x, y, z)
    }
}
