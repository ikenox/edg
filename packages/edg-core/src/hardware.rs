use crate::{Color, KeyIndex};

pub trait Keyboard {
    fn key_states(&self) -> &[(KeyIndex, bool)];
}

pub trait Led {
    fn set(&mut self, i: &KeyIndex, color: &Color);
}
