use yew::prelude::*;

pub struct ListGroup {
    props: ListGroupProps,
}

#[derive(Properties, Clone, PartialEq)]
pub struct ListGroupProps {
    pub children: Children,
}

impl Component for ListGroup {
    type Message = ();
    type Properties = ListGroupProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _msg: Self::Propertis) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <ul class="list-group">{ self.props.children.clone()}</ul>
        }
    }
}
