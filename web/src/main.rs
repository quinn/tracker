mod components;

use crate::components::App;
use yew::Renderer;

fn main() {
    let renderer = Renderer::<App>::new();

    renderer.hydrate();
}
