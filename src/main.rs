mod app;

fn main() {
    huiwen::logger::init(log::Level::Info);
    app::App::start_app();
}
