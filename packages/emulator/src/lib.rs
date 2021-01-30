use keymap::KeyMap;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    keys: i64,
}

enum Msg {
    AddOne,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, keys: 10 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                { for (0..self.keys).into_iter().map(|i|html!{<KeySwitch text=i.to_string() pressed=true/>}) }
            </div>
        }
    }
}
#[derive(Properties, Clone, PartialEq)]
pub struct KeySwitchProps {
    text: String,
    #[prop_or(false)]
    pressed: bool,
}
struct KeySwitch {
    props: KeySwitchProps,
    link: ComponentLink<Self>,
}
impl Component for KeySwitch {
    type Message = ();
    type Properties = KeySwitchProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        true
    }

    fn view(&self) -> Html {
        html! {
                <button>{ &self.props.text }</button>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
