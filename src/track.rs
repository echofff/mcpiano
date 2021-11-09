use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Default)]
pub struct Note {
    pub insi: u8,
    pub inst: u8,
    pub note: u8,
    pub beat: u8,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Default)]
pub struct Track {
    pub inst: usize,
    pub notes: Vec<Note>,
}

impl Track {
    pub fn new() -> Track {
        let notes = vec![Default::default()];
        let inst = 0;
        Track { inst, notes }
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
