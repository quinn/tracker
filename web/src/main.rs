mod components;

use crate::components::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
