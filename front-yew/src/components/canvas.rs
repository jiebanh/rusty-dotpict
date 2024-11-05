use std::f64;
use yew::prelude::*;
use web_sys::{HtmlElement, HtmlCanvasElement, CanvasRenderingContext2d};
// use js_sys;
// use web_sys::wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(512)]
    pub window_size: usize,
    #[prop_or(128)]
    pub pixel_count: usize,
    // pub colors: Vec::<>,
}

// TODO check this in MouseEvent
// #[derive(PartialEq)]
// enum State {
//     DOWN,
//     UP,
// }

#[function_component(CanvasDiv)]
pub fn canvas_div() -> Html {
    let class_div = "flex justify-center items-center m-1";
    html! {
        <div class={class_div}>
            <Canvas />
        </div>
    }
}

#[function_component(Canvas)]
pub fn canvas(props: &Props) -> Html {
    let Props {pixel_count, ..} = props;

    web_sys::console::log_1(&format!("pixel_count: {:?}", pixel_count).into());

    let canvas_ref = use_node_ref();
    // let canvas = canvas_ref.cast::<HtmlCanvasElement>().unwrap();
    // let ctx: CanvasRenderingContext2d = canvas.get_context("2d")
    //     .unwrap()
    //     .unwrap()
    //     .dyn_into()
    //     .unwrap();

    // this does not work. tailwind runs on build
    // let class_canvas = format!("w-[{}px] h-[{}px]", window_size, window_size);

    // let onmousemove = Callback::from(|e: MouseEvent| {
    let onmousedown = Callback::from(|e: MouseEvent| {
    // let ondrag = Callback::from(|e: DragEvent| {
        if let Some(target) = e.target_dyn_into::<HtmlElement>() {
            let rect = target.get_bounding_client_rect();
            let x = (e.client_x() as f64) - rect.left();
            let y = (e.client_y() as f64) - rect.top();
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

            ctx.arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
                .unwrap();
            ctx.stroke();
        }
        // let canvas = e.target().unwrap().dyn_ref::<HtmlCanvasElement>().unwrap()
        //     .map_err(|_| ())
        //     .unwrap();
        // ()
    });

    html! {
        // <canvas class="w-[128px] h-[128px]" {onmousemove}>
        // <canvas class="w-[512px] h-[512px]" {ondrag}>
        <canvas class="w-[512px] h-[512px]" {onmousedown} ref={canvas_ref}>
        // <canvas class={class_canvas} {onmousemove}>
        </canvas>
    }
}
