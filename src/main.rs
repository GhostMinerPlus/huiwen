use huiwen::page;

fn main() {
    let _ = console_log::init_with_level(log::Level::Debug);
    yew::Renderer::<page::Main>::new().render();
}
