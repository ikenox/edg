use keylistener::KeyListener;
use keymap::KeyMap;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::services::keyboard::{KeyListenerHandle, KeyboardService};
use yew::services::ConsoleService;
use yew::web_sys;

struct Model {
    link: ComponentLink<Self>,
    up_listener: KeyListenerHandle,
    down_listener: KeyListenerHandle,
    keys: HashMap<u32, bool>,
}

enum Msg {
    KeyDown(u32),
    KeyUp(u32),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let up_listener = KeyboardService::register_key_up(
            &web_sys::window().unwrap(),
            (&link).callback(|e: KeyboardEvent| Self::Message::KeyUp(e.key_code())),
        );
        let down_listener = KeyboardService::register_key_down(
            &web_sys::window().unwrap(),
            (&link).callback(|e: KeyboardEvent| Self::Message::KeyDown(e.key_code())),
        );
        Self {
            link,
            up_listener,
            down_listener,
            keys: (0..100).into_iter().map(|i| (i, false)).collect(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::KeyDown(c) => {
                if self.keys.get(&c).is_some() {
                    self.keys.insert(c, true);
                }
            }
            Msg::KeyUp(c) => {
                if self.keys.get(&c).is_some() {
                    self.keys.insert(c, false);
                }
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        let key_elems = self
            .keys
            .iter()
            .map(|(i, pressed)| html! {<KeySwitch pressed=pressed/>});
        html! {
            <div>
                { for key_elems }
                <KeyListener />
            </div>
        }
    }
}
#[derive(Properties, Clone, PartialEq)]
pub struct KeySwitchProps {
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
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        ConsoleService::info(format!("render key").as_str());
        html! {
                <button class=if self.props.pressed {"button-down"}else{""}>{"[ ]"}</button>
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
