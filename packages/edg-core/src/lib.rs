#![no_std]

pub mod hardware;

use arrayvec::ArrayVec;
use hashbrown::{HashMap, HashSet};

#[derive(Hash, Eq, PartialEq)]
pub struct KeyIndex(isize);

#[derive(Hash, Eq, PartialEq)]
pub enum KeyInteraction {
    Press,
    Tap,
}

#[derive(Hash, Eq, PartialEq)]
pub enum Action {
    SendKey(usize),
    // System(),
}

pub struct KeyMap {
    pub mappings: HashMap<HashMap<KeyIndex, KeyInteraction>, HashSet<Action>>,
}

pub struct Color(pub u8, pub u8, pub u8);

pub struct System {}
impl System {
    pub fn run() {
        loop {
            asy
        }
    }
}

// impl KeyMap {
//     pub fn resolve(&self, pushed: &HashMap<KeyIndex, KeyInteraction>) -> HashSet<Action> {}
// }

// 物理キーinteractionをstream送信 (pressed, pressing, unpressed, tap) -> interactionsに変換 -> actionsに変換

// 同時押し、タップ
