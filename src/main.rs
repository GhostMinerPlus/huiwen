use huiwen::app;

fn main() {
    let _ = console_log::init_with_level(log::Level::Info);
    yew::Renderer::<app::Main>::new().render();
}
