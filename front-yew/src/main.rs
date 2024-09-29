// use::yew::prelude::*;

mod components;
use crate::components::counter::Counter;

// mod nest_example;

fn main() {
    yew::Renderer::<Counter>::new().render();
}
