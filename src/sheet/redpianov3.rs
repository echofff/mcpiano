use std::ops::{Deref, DerefMut};

use serde_json::{json, Map, Value};
use wasm_bindgen::{throw_str, JsValue, UnwrapThrowExt};

use crate::draw::{Area, Draw};
use crate::event::{Event, Key, KeyCata};

use super::{CommonData, Sheet};

#[derive(Copy, Clone, Debug)]
struct NoteDe {
    note: usize,
    delay: usize,
}

impl NoteDe {
    pub fn new() -> Self {
        NoteDe { note: 25, delay: 0 }
    }
}

pub struct RedPianoV3 {
    notes: Vec<NoteDe>,
    len: usize,

    cd: CommonData,
}

impl RedPianoV3 {
    pub fn new() -> Self {
        RedPianoV3 {
            notes: vec![NoteDe::new()],
            len: 4,
            cd: CommonData {
                sel_inst: 11,
                tpm: 2,
            },
        }
    }
}

impl Sheet for RedPianoV3 {
    fn tr_len(&self) -> usize {
        //self.len

        1
    }
    fn click(&mut self, event: Event) -> bool {
        match event {
            Event {
                area: Area::EditPlane,
                cata: KeyCata::Down,
                key: Key::Left,
                xi,
                yi,
                ctrl,
                ..
            } => {
                let ni = xi >> 2;
                let len = self.notes.len();
                self.resize(ni);
                //crate::alert(format!("--click------").as_str());
                if let Some((i, n)) = self
                    .notes
                    .iter_mut()
                    .rev()
                    .skip(len - ni - 1)
                    .take(4)
                    //.inspect(|n| crate::alert(format!("--{:?}--", n).as_str()))
                    .enumerate()
                    .find(|(_, n)| n.note > 24)
                {
                    n.note = 24 - yi;
                    n.delay = i * 4 + (xi & 0b11);
                    //crate::alert(format!("--click{:?}--", n,).as_str());
                }
            }
            _ => return false,
        }
        false
    }

    fn draw(&self, c: &Draw) {
        self.notes.iter().enumerate().for_each(|(i, n)| {
            if n.note < 25 {
                c.rect(i * 4 + n.delay + 4, 24 - n.note + 1, 4, 1, true);
            }
        })
    }

    fn save(&self) -> String {
        todo!()
    }
    fn load(&mut self, str: String) {
        todo!()
    }

    fn save_comp(&self) -> String {
        todo!()
    }

    fn add_inst(&mut self, inst: usize, color_s: String) {
        todo!()
    }

    fn resize(&mut self, tar: usize) -> usize {
        if self.notes.len() < tar {
            self.notes.resize(tar, NoteDe::new());
            //crate::alert(format!("--{}--", tar).as_str());
        };
        tar
    }
    fn time(&self) -> usize {
        self.notes.len() * 4
    }

    fn key(&mut self, x: usize, y: usize, key: usize) -> Option<(usize, usize)> {
        todo!()
    }
}

impl Deref for RedPianoV3 {
    type Target = CommonData;

    fn deref(&self) -> &Self::Target {
        &self.cd
    }
}
impl DerefMut for RedPianoV3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cd
    }
}
