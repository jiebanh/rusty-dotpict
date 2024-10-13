use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[derive(Clone, Routable, PartialEq)]
pub enum PaintRoute {
    #[at("/paint")]
    Paint,
    #[not_found]
    #[at("/settings/404")]
    NotFound,
}

// routing
pub fn switch_paint(route: PaintRoute) -> Html {
    match route {
        PaintRoute::Paint => html! {<Paint/>},
        PaintRoute::NotFound => html! {<Redirect<Route> to={Route::NotFound}/>},
    }
}

#[function_component]
fn Paint() -> Html {
    let style_string = "color: blue";
    // let onclick = Callback::from(|_| style_string = "color: red");
    html! {
        <div>
            <h1 style={style_string}>{"This is paint page"}</h1>
            // <button {onclick}>{ "redder" }</button>
            <Navbar />
            <Container />
        </div>
    }
}

#[function_component]
fn Navbar() -> Html {
    html! {
        <div class="navbar">
            <button class="btn">{ "Reset" }</button>
            <input type="color" value="#00eeff" class="color" />
            <input type="number" value="30" class="size" />
        </div>
    }
}

#[function_component]
fn Container() -> Html {
    html! {
        <div class="container">
        </div>
    }
}
