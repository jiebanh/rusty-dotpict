use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::HtmlInputElement;
use web_sys::wasm_bindgen::prelude::*;

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

#[derive(Clone, Debug, PartialEq)]
struct PaintContext {
    current_color: String,
    current_color_index: usize,
    color_palette: Vec::<String>,
    colors: Vec::<Vec::<usize>>,
}

#[function_component]
fn Paint() -> Html {
    let class = "text-blue-800 justify-center items-center flex";

    // using context (or just pass single state as prop)
    let ctx = use_state(|| PaintContext {
        current_color: String::from("#000000"),
        current_color_index: 0,
        color_palette: vec![String::from("#000000"); 16],
        colors: Vec::<Vec::<usize>>::new(),
    });

    let navigator_tohome = use_navigator().unwrap();
    let onclick_tohome = Callback::from(move |_| navigator_tohome.push(&Route::Home));

    let oninput = Callback::from({
        let ctx = ctx.clone();

        move |e: InputEvent| {
            // let PaintContext {current_color_index, color_palette, color

            let target: HtmlInputElement = e
                .target()
                .unwrap()
                .dyn_into()
                .unwrap();
            ctx.set(PaintContext {
                current_color: target.value(),
                current_color_index: ctx.current_color_index.clone(),
                color_palette: ctx.color_palette.clone(),
                colors: ctx.colors.clone(),
            });
        }
    });

    html! {
        <ContextProvider<PaintContext> context={(*ctx).clone()}>
            <div>
                <h1 {class}>{"This is paint page"}</h1>
                <buttons::Sample text="home" onclick={onclick_tohome} />
                <Navbar current_color={(ctx.current_color).clone()} {oninput} />
                <canvas::Canvas current_color={(ctx.current_color).clone()} />
            </div>
        </ContextProvider<PaintContext>>
    }
}

// sub components

#[derive(Properties, PartialEq)]
struct NavbarProps {
    current_color: String,
    oninput: Callback<InputEvent>,
}

#[function_component]
fn Navbar(props: &NavbarProps) -> Html {
    // NOTE concat!("a", "b") can be used to break line
    let class = "bg-black-navbar w-[800px] rounded-[3px] \
        p-1 mb-1 flex justify-center items-center";

    let NavbarProps{ current_color, oninput } = props;

    html! {
        <div {class}>
            <button class="btn">{ "Reset" }</button>
            <input type="color" value={String::from(current_color)} {oninput} class="color" />
            <input type="number" value="30" class="size" />
        </div>
    }
}
