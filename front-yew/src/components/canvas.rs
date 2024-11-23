use std::f64;
use yew::prelude::*;
use web_sys::{HtmlElement, HtmlCanvasElement, CanvasRenderingContext2d};
// use js_sys;
// use web_sys::wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::prelude::*;

// fn create_default_colors() -> Vec::<String> {
//     Vec::<String>::new()
// }

// fn get_cell(mouse_x: f64, mouse_y: f64, window_size: usize, pixel_count: usize) -> () {

// }

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(String::from("512"))]
    pub window_size: String,
    #[prop_or(128)]
    pub pixel_count: usize,
    // pub colors: Vec::<>,
    pub current_color: String,
}

// TODO check this in MouseEvent
// #[derive(PartialEq)]
// enum State {
//     DOWN,
//     UP,
// }

// #[function_component(CanvasDiv)]
// pub fn canvas_div() -> Html {
//     let class_div = "flex justify-center items-center m-1";
//     html! {
//         <div class={class_div}>
//             <Canvas />
//         </div>
//     }
// }

#[function_component(Canvas)]
pub fn canvas(props: &Props) -> Html {
    let Props {pixel_count, window_size, current_color, ..} = props;
    let class_div = "flex justify-center items-center m-1";
    let mouse_down = use_state(|| false);

    let color_borrowed = current_color.clone();
    let window_size_borrowed = window_size.clone();

    web_sys::console::log_1(&format!("pixel_count: {:?}", pixel_count).into());

    // let canvas_ref = use_node_ref();

    // let canvas = canvas_ref.cast::<HtmlCanvasElement>().unwrap();
    // let ctx: CanvasRenderingContext2d = canvas.get_context("2d")
    //     .unwrap()
    //     .unwrap()
    //     .dyn_into()
    //     .unwrap();

    // this does not work. tailwind runs on build
    // let class_canvas = format!("w-[{}px] h-[{}px]", window_size, window_size);

    // let onmousedown = Callback::from(
    //     move |_| {
    //         let mouse_down = mouse_down.clone();
    //         mouse_down.set(true);
    //     }
    // );

    let onload = Callback::from(move |e: Event| {
        web_sys::console::log_1(&format!("{}", "onshow running").into());
        if let Some(canvas) = e.target_dyn_into::<HtmlCanvasElement>() {
            let window_size_f: f64 = window_size_borrowed.to_owned().parse().unwrap();
            let ctx = canvas.get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();

            ctx.set_fill_style(&"#000000".into());
            ctx.fill_rect(0 as f64, 0 as f64, window_size_f, window_size_f);
        }
    });

    let onmousedown = {
        let mouse_down = mouse_down.clone();
        Callback::from(move |_| {
            mouse_down.set(true);
        })
    };

    let onmouseup = {
        let mouse_down = mouse_down.clone();
        Callback::from(move |_| {
            mouse_down.set(false);
        })
    };

    let onmousemove = Callback::from(move |e: MouseEvent| {
        if *mouse_down == false {
            return
        }

        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let color_borrowed = color_borrowed.to_owned();

        if let Some(target) = e.target_dyn_into::<HtmlElement>() {
            let rect = target.get_bounding_client_rect();

            x = (e.client_x() as f64) - rect.left();
            y = (e.client_y() as f64) - rect.top();
            web_sys::console::log_1(&format!("RectLeft? : {} ; RectTop? : {}", rect.left(), rect.top()).into());
            web_sys::console::log_1(&format!("Left? : {} ; Top? : {}", x, y).into());
        }

        if let Some(canvas) = e.target_dyn_into::<HtmlCanvasElement>() {
            // let context_object: js_sys::Object = canvas.get_context("2d")
            //     .unwrap()
            //     .unwrap();
            // let ctx = context_object.dyn_into::<CanvasRenderingContext2d>()
            //     .unwrap();

            let ctx = canvas.get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();

            // ctx.set_fill_style(&"#0000ff".into());
            ctx.set_fill_style(&color_borrowed.into());

            // ctx.set_fill_style(&"rgb(150,50,0)".into());

            ctx.fill_rect(x.round(), y.round(), 30.0, 30.0);
            web_sys::console::log_1(&format!("x? : {} ; y? : {}", x, y).into());
            // ctx.fill_rect(75.0, 75.0, 30.0, 30.0);

            // ctx.arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
            //     .unwrap();
            // ctx.stroke();
        }
        // let canvas = e.target().unwrap().dyn_ref::<HtmlCanvasElement>().unwrap()
        //     .map_err(|_| ())
        //     .unwrap();
        // ()
    });

    // NOTE tailwind px (class="w-[512px] h-[512px]") gives wrong result to rill_rect

    html! {
        <div class={class_div}>
            // <canvas class="w-[128px] h-[128px]" {onmousemove}>
            // <canvas class="w-[512px] h-[512px]" {ondrag}>
            // <canvas class={class_canvas} {onmousemove}>
            <canvas style="border:1px solid #000000" width={window_size.clone()} height={window_size.clone()}
                {onload} {onmouseup} {onmousedown} {onmousemove}>
            </canvas>
        </div>
    }
}
