use keylistener::KeyListener;
use keymap::KeyMap;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::services::keyboard::{KeyListenerHandle, KeyboardService};
use yew::web_sys;

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
        Self { link, keys: 300 }
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
                { for (0..self.keys).into_iter().map(|i|html!{<KeySwitch keyindex=i/>}) }
                <KeyListener />
            </div>
        }
    }
}
#[derive(Properties, Clone, PartialEq)]
pub struct KeySwitchProps {
    keyindex: i64,
}
struct KeySwitch {
    props: KeySwitchProps,
    link: ComponentLink<Self>,
    up_listener: KeyListenerHandle,
    down_listener: KeyListenerHandle,
    pressed: bool,
}
enum KeySwitchEvent {
    Down(u32),
    Up(u32),
}
impl Component for KeySwitch {
    type Message = KeySwitchEvent;
    type Properties = KeySwitchProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let up_listener = KeyboardService::register_key_up(
            &web_sys::window().unwrap(),
            (&link).callback(|e: KeyboardEvent| Self::Message::Up(e.key_code())),
        );
        let down_listener = KeyboardService::register_key_down(
            &web_sys::window().unwrap(),
            (&link).callback(|e: KeyboardEvent| Self::Message::Down(e.key_code())),
        );
        Self {
            props,
            link,
            up_listener,
            down_listener,
            pressed: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            KeySwitchEvent::Down(c) => {
                if c == self.props.keyindex as u32 {
                    self.pressed = true;
                    true
                } else {
                    false
                }
            }
            KeySwitchEvent::Up(c) => {
                if c == self.props.keyindex as u32 {
                    self.pressed = false;
                    true
                } else {
                    false
                }
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        true
    }

    fn view(&self) -> Html {
        html! {
                <button class=if self.pressed {"button-down"}else{""}>{ self.props.keyindex }</button>
        }
    }
}

pub mod keylistener {
    use itertools::Itertools;
    use std::collections::{BTreeSet, HashSet};
    use wasm_bindgen::prelude::*;
    use yew::prelude::*;
    use yew::services::keyboard::{KeyListenerHandle, KeyboardService};
    use yew::web_sys;

    #[derive(Properties, Clone, PartialEq)]
    pub struct Props {}
    pub struct KeyListener {
        props: Props,
        link: ComponentLink<Self>,
        up_listener: KeyListenerHandle,
        down_listener: KeyListenerHandle,
        pressed_keys: BTreeSet<u32>,
    }
    pub enum Msg {
        Down(u32),
        Up(u32),
    }
    impl Component for KeyListener {
        type Message = Msg;
        type Properties = Props;

        fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
            let up_listener = KeyboardService::register_key_up(
                &web_sys::window().unwrap(),
                (&link).callback(|e: KeyboardEvent| Self::Message::Up(e.key_code())),
            );
            let down_listener = KeyboardService::register_key_down(
                &web_sys::window().unwrap(),
                (&link).callback(|e: KeyboardEvent| Self::Message::Down(e.key_code())),
            );
            Self {
                props,
                link,
                up_listener,
                down_listener,
                pressed_keys: BTreeSet::new(),
            }
        }

        fn update(&mut self, msg: Self::Message) -> bool {
            match msg {
                Msg::Down(c) => self.pressed_keys.insert(c),
                Msg::Up(c) => self.pressed_keys.remove(&c),
            };
            true
        }

        fn change(&mut self, props: Self::Properties) -> bool {
            true
        }

        fn view(&self) -> Html {
            let txt = self
                .pressed_keys
                .iter()
                .map(|c| format!("{}", c))
                .join(", ");
            html! {
            <div>
                { txt }
            </div>
            }
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
