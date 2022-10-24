mod apis;
mod components;
mod models;

use crate::components::App;
use yew::Renderer;

fn main() {
    let renderer = Renderer::<App>::new();

    renderer.hydrate();
}
