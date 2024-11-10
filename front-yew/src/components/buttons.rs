use yew::prelude::*;

// pub enum Style {
//     Basic,
//     Color1,
// }

// impl Style {
//     pub fn as_str(&self) -> &'static str {
//         match self {
//             Style::Basic => "p-4 rounded-lg bg-blue-400 hover:bg-blue-500 font-bold text-white shadow-lg shadow-blue-200 transition ease-in-out duration-200 translate-10",
//             Style::Color1 => "flex max-w-sm w-full bg-gradient-to-r from-indigo-500 via-pink-500 to-yellow-500 hover:from-indigo-600 hover:via-pink-600 hover:to-red-600 focus:outline-none text-white text-2xl uppercase font-bold shadow-md mx-auto p-5"
//         }
//     }
// }

#[derive(Properties, PartialEq)]
pub struct Props {
    pub text: String,
    pub onclick: Callback<MouseEvent>,

    // #[prop_or_default]
    // pub is_loading: bool,
    // #[prop_or("Bob".to_string())]
    // pub prop_a: String,
    // #[prop_or_else(some_function)]
    // pub name: String,
}

#[function_component(Sample)]
pub fn sample(props: &Props) -> Html {
    let Props { text, onclick } = props;
    let class = "flex max-w-sm w-full bg-gradient-to-r \
        from-indigo-500 via-pink-500 to-yellow-500 \
        hover:from-indigo-600 hover:via-pink-600 hover:to-red-600 \
        focus:outline-none text-white text-2xl uppercase \
        font-bold shadow-md mx-auto p-5";
    html! {
        <button {class} onclick={onclick}>{ text }</button>
    }
}

#[function_component(Basic)]
pub fn basic(props: &Props) -> Html {
    let class = "p-4 rounded-lg bg-blue-400 hover:bg-blue-500 \
        font-bold text-white shadow-lg shadow-blue-200 \
        transition ease-in-out duration-200 translate-10";

    html! {
        <button {class} onclick={&props.onclick}>{ &props.text }</button>
    }
}

// use state outside could be better

// #[function_component(WithState)]
// pub fn basic() -> Html {
//     let counter = use_state(|| 0);
//     let onclick = {
//         let counter = counter.clone();
//         move|_| {
//             let value = *counter + 1;
//             counter.set(value);
//         }
//     };

//     let class = "p-4 rounded-lg bg-blue-400 hover:bg-blue-500 \
//         font-bold text-white shadow-lg shadow-blue-200 \
//         transition ease-in-out duration-200 translate-10";

//     html! {
//         <button {class} {onclick}>{ *counter }</button>
//     }
// }

// #[function_component(WithProp)]
// pub fn basic(props: &Props) -> Html {
//     let class = "p-4 rounded-lg bg-blue-400 hover:bg-blue-500 \
//         font-bold text-white shadow-lg shadow-blue-200 \
//         transition ease-in-out duration-200 translate-10";

//     html! {
//         <button {class} onclick={&props.onclick}>{ &props.text }</button>
//     }
// }
