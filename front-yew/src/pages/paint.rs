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
    colors: Vec::<String>,
}

#[function_component]
fn Paint() -> Html {
    // let style_string = "text-blue-500 center";
    let class = "text-blue-800 justify-center items-center flex";
    // let onclick = Callback::from(|_| style_string = "color: red");

    // let ctx = use_state(|| ColorContext {
    //     current_color: String::from("#000000"),
    //     colors: Vec::<String>::new(),
    // });

    // let colors = use_state(|| vec::<String>);
    let current_color = use_state(|| String::from("#ff0000"));
    // let current_color = use_state(|| "#ff0000");

    let navigator_tohome = use_navigator().unwrap();
    let onclick_tohome = Callback::from(move |_| navigator_tohome.push(&Route::Home));

    // let onchange = {
    //     let current_color = current_color.clone();
    //     Callback::from(move | e: Event | {
    //         let target = e.target().unwrap();
    //         let current_value = target.unchecked_into::<HtmlInputElement>();
    //         // temp_value = current_value.value();
    //         current_color.set(&current_value.value());
    //     })
    // };

    let oninput = Callback::from({
        let current_color = current_color.clone();
        move |e: InputEvent| {
            let target: HtmlInputElement = e
                .target()
                .unwrap()
                .dyn_into()
                .unwrap();
            current_color.set(target.value());
        }
    });

    html! {
        // <ContextProvider<ColorContext> context={(*ctx).clone()}>
        <div>
            <h1 {class}>{"This is paint page"}</h1>
            // <button {onclick}>{ "redder" }</button>
            <buttons::Sample text="home" onclick={onclick_tohome} />
            // <Navbar current_color={*current_color} {onchange} />
            <Navbar current_color={(*current_color).clone()} {oninput} />
            // <canvas::CanvasDiv />
            <canvas::Canvas current_color={(*current_color).clone()} />
        </div>
        // </ContextProvider<ColorContext>>
    }
}

// TODO write in main component directly, since this is not reused
// or recieve callback function as prop
// sub components

#[derive(Properties, PartialEq)]
struct NavbarProps {
    current_color: String,
    // onchange: Callback<Event>,
    oninput: Callback<InputEvent>,
}

#[function_component]
fn Navbar(props: &NavbarProps) -> Html {
    // there is concat!("a", "b") macro also
    let class = "bg-black-navbar w-[800px] rounded-[3px] \
        p-1 mb-1 flex justify-center items-center";

    let NavbarProps{ current_color, oninput } = props;
    // let onchange = Callback::from(
    //     move | input_event: Event | {
    //         let input_event_target = input_event.target().unwrap();
    //         let current_value = input_event_target.unchecked_into::<HtmlInputElement>();
    //         current_color = &(current_value.value()).into();
    //     }
    // );

    html! {
        <div {class}>
            <button class="btn">{ "Reset" }</button>
            <input type="color" value={String::from(current_color)} {oninput} class="color" />
            <input type="number" value="30" class="size" />
        </div>
    }
}
