use std::ops::{Deref, DerefMut};

use crate::*;
use wasm_bindgen::prelude::*;
use web_sys::{AudioBuffer, AudioContext, Request, RequestInit, Response};

#[wasm_bindgen]
pub struct PianoGlobal {

    #[wasm_bindgen(skip)]
    pub actx: Player,
    //tracks: Vec<Track>,
    #[wasm_bindgen(skip)]
    pub sheet: Box<dyn Sheet>,

    #[wasm_bindgen(skip)]
    pub rt: RuntimeData,

    #[wasm_bindgen(skip)]
    pub theme: Theme,

    #[wasm_bindgen(skip)]
    pub cctx: Draw,
}

pub struct RuntimeData {
    //pub sel_inst: usize,
    //pub maxnote: usize,
    pub pos: (usize, usize),

    // u8 is beat, in fromat 0bxxxx
    pub error_hl: Vec<(usize, u8)>,
    pub select_hl: Vec<(usize, u8)>,

    pub play_bt: usize,
}

impl Deref for PianoGlobal {
    type Target = RuntimeData;

    fn deref(&self) -> &Self::Target {
        &self.rt
    }
}

impl DerefMut for PianoGlobal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rt
    }
}

pub struct Theme {
    pub control: [String; 6],
    pub track_row: Vec<String>,
    pub note_row: Vec<String>,

    pub sel: String,
    pub error: String,
    pub play: String,

    pub hover: String,
}

impl Theme {
    pub fn new() -> Theme {
        Theme {
            control: [
                "green".into(),
                "blue".into(),
                "red".into(),
                "oraange".into(),
                "yellow".into(),
                "red".into(),
            ],
            track_row: vec!["#22443322".into(), "#44223333".into()],
            note_row: vec![
                "#aaaaaa".into(),
                "#ffffff".into(),
                "#aaaaaa".into(),
                "#ffffff".into(),
                "#aaaaaa".into(),
                "#ffffff".into(),
                "#ffffff".into(),
                "#aaaaaa".into(),
                "#ffffff".into(),
                "#aaaaaa".into(),
                "#ffffff".into(),
                "#ffffff".into(),
            ],
            sel: "#2222cc4".into(),
            error: "#ff4444c4".into(),
            play: "#44cc44c4".into(),
            hover: "#66cccc88".into(),
        }
    }
}

impl RuntimeData {
    pub fn new() -> RuntimeData {
        RuntimeData {
            //sel_inst: 11,
            //maxnote: 20,
            pos: (0, 0),

            error_hl: Vec::new(),
            select_hl: Vec::new(),
            play_bt: 0,
        }
    }
}
