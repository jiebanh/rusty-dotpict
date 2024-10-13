use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[derive(Clone, Routable, PartialEq)]
pub enum TailwindRoute {
    #[at("/tailwind")]
    Tailwind,
    #[not_found]
    #[at("/settings/404")]
    NotFound,
}

#[function_component(Tailwind)]
fn tailwind() -> Html {
    let link_classes =
        "block px-4 py-2 hover:bg-black hover:text-white rounded border-black border";
    let links = [
        ("Trunk", "https://github.com/trunk-rs/trunk"),
        ("Yew", "https://yew.rs/"),
        ("Tailwind", "https://tailwindcss.com"),
    ];

    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::Home));

    html! {
        <div>
            <div class="flex flex-col h-screen">
                <nav class="bg-green-400 h-16 px-8 py-2">
                    <div class="container flex mx-auto gap-6 items-center h-full">
                        <h1 class="font-bold text-2xl text-white">{"Trunk | Yew | Tailwind"}</h1>
                        <div class="flex-1"></div>
                        // sample causing error
                        // {
                        //     for links.iter().map(|(label, href)| {
                        //         html! {
                        //             <a class={link_classes} href={href}>{label}</a>
                        //         }
                        //     })
                        // }
                        {
                            links.into_iter().map(|(label, href)| {
                                html! {
                                    <a class={link_classes} href={href}>{label}</a>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </nav>
                <div class = "bg-gray-50 flex-1 flwx py-16 px-8 items-center gap-6 justify-center">
                    {view_card("Trunk", None, html! {
                        <p>{"Trunk is a WASM web application bundler for Rust."}</p>
                    })}
                    {view_card("Yew", Some("static/images/test.dio.svg"), html! {
                        <p>{"Yew is a modern Rust framework for creating multi-threaded front-end web apps with WebAssembly."}</p>
                    })}
                    {view_card("Tailwind CSS", None, html! {
                        <p>{"Tailwind CSS is a library for styling markup using a comprehensive set of utility classes, no CSS required."}</p>
                    })}
                </div>
            </div>
            <div>
                <h1>{"this is Tailwind sample page"}</h1>
                <button {onclick}>{ "home" }</button>
            </div>
        </div>
    }
}

fn view_card(title: &'static str, img_url: Option<&'static str>, content: Html) -> Html {
    html! {
        <div class="w-96 h-48 rounded bg-green-400 text-white p-6">
            {
                img_url.map(|url| html! {
                    <img class="float-right w-12" src={url} alt="Logo" />
            })}
            <h1 class="text-4xl mb-8">{title}</h1>
            {content}
        </div>
    }
}

pub fn switch_tailwind(route: TailwindRoute) -> Html {
    match route {
        TailwindRoute::Tailwind => html! {<Tailwind />},
        TailwindRoute::NotFound => html! {<Redirect<Route> to={Route::NotFound}/>},
    }
}
