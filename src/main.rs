mod element;
mod route;
mod service;

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Info).unwrap();

    App::start_app();
}

struct App {
    pub(crate) name: String,
}

impl App {
    fn new() -> Self {
        Self {
            name: "huiwen".to_string(),
        }
    }
}

static mut APP: Option<App> = None;

impl App {
    pub(crate) fn get_app() -> &'static App {
        unsafe { APP.as_ref().unwrap() }
    }

    fn start_app() {
        unsafe {
            APP = Some(App::new());
            APP.as_mut().unwrap().run();
        }
    }

    fn run(&mut self) {
        yew::Renderer::<element::Main>::new().render();
    }
}
