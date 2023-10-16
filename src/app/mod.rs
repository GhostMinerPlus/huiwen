mod page;
mod route;

pub struct App {
    pub name: String,
}

static mut APP: Option<App> = None;

impl App {
    fn new() -> Self {
        crate::api::init();
        Self {
            name: "huiwen".to_string(),
        }
    }
}

impl App {
    pub fn get_app() -> &'static App {
        unsafe { APP.as_ref().unwrap() }
    }

    pub fn start_app() {
        unsafe {
            APP = Some(App::new());
            APP.as_mut().unwrap().run();
        }
    }

    fn run(&mut self) {
        yew::Renderer::<page::Main>::new().render();
    }
}