use std::ops::{Deref, DerefMut};

use wasm_bindgen::JsValue;

use super::Sheet;

pub struct RedPianoV2 {
    tracks: Vec<Track>,
}

impl RedPianoV2 {
    pub fn new() -> RedPianoV2 {
        RedPianoV2 {
            tracks: vec![Track::new()],
        }
    }
}

impl Sheet for RedPianoV2 {
    fn tr_len(&self) -> usize {
        self.tracks.len()
    }

    fn colo(&self,n: usize) -> usize {
        &self.tracks.
    }
}

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
        self.iter()
            .enumerate()
            .rev()
            .find(|(_, n)| n.beat != 0)
            .map(|(i, _)| i + 1)
            .unwrap_or(0)
    }
    pub fn gn(&self, ti: usize) -> Option<&Note> {
        self.get(ti >> 2)
    }
    pub fn gnb(&self, ti: usize) -> Option<(&Note, u8)> {
        self.get(ti >> 2).map(|n| (n, ti.to_beat()))
    }
    pub fn gnb_mut(&mut self, ti: usize) -> Option<(&mut Note, u8)> {
        self.get_mut(ti >> 2).map(|n| (n, ti.to_beat()))
    }
    pub fn gn_mut(&mut self, ti: usize) -> Option<&mut Note> {
        self.get_mut(ti >> 2)
    }
}

trait BeatIndex {
    fn to_beat(&self) -> u8;
}

impl BeatIndex for usize {
    fn to_beat(&self) -> u8 {
        0b11 >> self & 0b11
    }
}

impl Deref for RedPianoV2 {
    type Target = Vec<Track>;

    fn deref(&self) -> &Self::Target {
        &self.tracks
    }
}
impl DerefMut for RedPianoV2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tracks
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
