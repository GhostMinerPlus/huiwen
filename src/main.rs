mod component;
mod element;

mod page;
mod router;
mod service;

mod util;

fn main() {
    let _ = console_log::init_with_level(log::Level::Info);
    yew::Renderer::<page::Main>::new().render();
}
