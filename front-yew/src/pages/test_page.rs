use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::components::counter::Counter;

#[derive(Clone, Routable, PartialEq)]
pub enum SettingsRoute {
    #[at("/settings")]
    Profile,
    #[at("/settings/friends")]
    Friends,
    #[not_found]
    #[at("/settings/404")]
    NotFound,
}

// it actually seems to define Profile struct and implement necessary traits
// and function defines view
#[function_component(Profile)]
fn profile() -> Html {
    html! {
        <div>
            <h1>{"this is Profile page"}</h1>
            <Secure />
        </div>
    }
}

#[function_component(Friends)]
fn friends() -> Html {
    html! {
        <div class={classes!("class-1", "class-2")}>
            <h1 style="color: red">{"this is Friends page"}</h1>
            <Counter />
            <Secure />
        </div>
    }
}

#[function_component(Secure)]
fn secure() -> Html {
    let navigator = use_navigator().unwrap();

    // using short form of onclick={onclick}
    let onclick = Callback::from(move |_| navigator.push(&Route::Home));

    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}

pub fn switch_settings(route: SettingsRoute) -> Html {
    match route {
        SettingsRoute::Profile => html! {<Profile />},
        SettingsRoute::Friends => html! {<Friends />},
        SettingsRoute::NotFound => html! {<Redirect<Route> to={Route::NotFound}/>},
    }
}
