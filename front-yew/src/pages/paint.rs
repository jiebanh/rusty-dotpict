use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::components::buttons;
use crate::components::canvas;

// routing

#[derive(Clone, Routable, PartialEq)]
pub enum PaintRoute {
    #[at("/paint")]
    Paint,
    #[not_found]
    #[at("/settings/404")]
    NotFound,
}

pub fn switch_paint(route: PaintRoute) -> Html {
    match route {
        PaintRoute::Paint => html! {<Paint/>},
        PaintRoute::NotFound => html! {<Redirect<Route> to={Route::NotFound}/>},
    }
}

// main component

#[function_component]
fn Paint() -> Html {
    // let style_string = "text-blue-500 center";
    let class = "text-blue-800 justify-center items-center flex";
    // let onclick = Callback::from(|_| style_string = "color: red");

    let navigator_tohome = use_navigator().unwrap();
    let onclick_tohome = Callback::from(move |_| navigator_tohome.push(&Route::Home));

    html! {
        <div>
            <h1 {class}>{"This is paint page"}</h1>
            // <button {onclick}>{ "redder" }</button>
            <buttons::Sample text="home" onclick={onclick_tohome} />
            <Navbar />
            <canvas::CanvasDiv />
            // <Container />
        </div>
    }
}

// sub components

#[function_component]
fn Navbar() -> Html {
    // there is concat!("a", "b") macro also
    let class = "bg-black-navbar w-[800px] rounded-[3px] \
        p-1 mb-1 flex justify-center items-center";

    html! {
        <div {class}>
            <button class="btn">{ "Reset" }</button>
            <input type="color" value="#00eeff" class="color" />
            <input type="number" value="30" class="size" />
        </div>
    }
}

#[function_component]
fn Container() -> Html {
    let class = "bg-black-navbar w-[800px] rounded-[3px] \
        h-[800px] grid grid-cols-4 grid-rows-4 gap-[3px] p-3";
    html! {
        <div {class}>
        </div>
    }
}
