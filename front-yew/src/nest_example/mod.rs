mod list;

use list::ListGroup;
use list::ListGroupItem;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _msg: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <ListGroup>
                <ListGroupItem>{"First"}</ListGroupItem>
                <ListGroupItem active=true>{"Second"}</ListGroupItem>
                <ListGroupItem>{"Third"}</ListGroupItem>
            </ListGroup>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
