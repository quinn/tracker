#[macro_use]
extern crate serde_derive;

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

mod apis;
mod components;
mod models;

use crate::components::App;
use yew::Renderer;

fn main() {
    let renderer = Renderer::<App>::new();

    renderer.hydrate();
}
