mod component;
mod element;

mod page;
mod router;

mod util;

fn main() {
    yew::Renderer::<page::Main>::new().render();
}
