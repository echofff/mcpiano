use std::ops::{Deref, DerefMut};

use wasm_bindgen::JsValue;

#[derive(Clone, serde::Deserialize, serde::Serialize, Default)]
pub struct Note {
    pub note: u8,
    pub beat: u8,
}

#[derive(Clone, serde::Deserialize, serde::Serialize, Default)]
pub struct Track {
    pub inst: usize,
    pub hide: bool,

    #[serde(skip_deserializing, skip_serializing)]
    pub colo: JsValue,

    pub colo_s: String,

    pub notes: Vec<Note>,
}

impl Track {
    pub fn new() -> Track {
        let notes = vec![Default::default()];
        let inst = 11;
        let hide = false;
        let colo = JsValue::from_str("#9a9dea");
        let colo_s = String::from("#9a9dea");
        Track {
            inst,
            hide,
            notes,
            colo,
            colo_s,
        }
    }
    pub fn deleteable(&self) -> bool {
        self.iter().all(|n| n.beat == 0)
    }
    pub fn true_len(&self) -> usize {
        if let Some((i, _)) = self.iter().enumerate().rev().find(|(_, n)| n.beat != 0) {
            i + 1
        } else {
            0
        }
    }
}

impl Deref for Track {
    type Target = Vec<Note>;

    fn deref(&self) -> &Self::Target {
        &self.notes
    }
}
impl DerefMut for Track {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.notes
    }
}
