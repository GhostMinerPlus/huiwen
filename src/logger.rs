pub fn init(lf: log::Level) {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(lf).expect("Couldn't initialize logger");
}
