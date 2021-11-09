use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Default)]
pub struct Note {
    pub inst: u8,
    pub note: u8,
    pub beat: u8,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Default)]
pub struct Track {
    pub inst: usize,
    pub hide: bool,
    pub colo: String,
    pub notes: Vec<Note>,
}

impl Track {
    pub fn new() -> Track {
        let notes = vec![Default::default()];
        let inst = 0;
        let hide = false;
        let colo = String::from("#9a9dea");
        Track {
            inst,
            hide,
            notes,
            colo,
        }
    }
    pub fn deleteable(&self) -> bool {
        self.iter().all(|n| n.beat == 0)
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
