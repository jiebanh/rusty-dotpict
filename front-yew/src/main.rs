use::yew::prelude::*;
use yew_router::prelude::*;
use components::buttons;

mod components;
mod pages;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/paint")]
    PaintRoot,
    #[at("/paint/*")]
    Paint,
    #[at("/settings")]
    SettingsRoot,
    #[at("/settings/*")]
    Settings,
    #[at("/tailwind")]
    TailwindRoot,
    #[at("/tailwind/*")]
    Tailwind,
    #[not_found]
    #[at("/404")]
    NotFound,
}

// can use
// #[function_component]
// fn Home() -> ...
#[function_component(Home)]
fn home() -> Html {
    html! {
        <div class="bg-red-100">
            <h1>{ "Home" }</h1>
            <Pages />
        </div>
    }
}

#[function_component(Pages)]
fn pages() -> Html {
    let navigator_paint = use_navigator().unwrap();
    let onclick_paint = Callback::from(move |_| navigator_paint.push(&Route::PaintRoot));

    let navigator_settings = use_navigator().unwrap();
    let onclick_settings = Callback::from(move |_| navigator_settings.push(&Route::SettingsRoot));

    let navigator_tailwind = use_navigator().unwrap();
    let onclick_tailwind = Callback::from(move |_| navigator_tailwind.push(&Route::TailwindRoot));

    // if function name is 'onclick', onclick={onclick} can be abbreviated to {onclick}
    html! {
        <div>
            <button class={buttons::Style::Basic.as_str()} onclick={onclick_paint}>{ "paint" }</button>
            <button class={buttons::Style::Color1.as_str()} onclick={onclick_settings}>{ "settings" }</button>
            <button onclick={onclick_tailwind}>{ "tailwind" }</button>
        </div>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::PaintRoot | Route::Paint => html! {
            <Switch<pages::paint::PaintRoute> render={pages::paint::switch_paint} />
        },
        Route::SettingsRoot | Route::Settings => html! {
            <Switch<pages::test_page::SettingsRoute> render={pages::test_page::switch_settings} />
        },
        Route::TailwindRoot | Route::Tailwind => html! {
            <Switch<pages::tailwind_sample::TailwindRoute> render={pages::tailwind_sample::switch_tailwind} />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
