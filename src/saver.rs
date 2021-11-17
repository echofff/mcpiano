use serde::{Deserialize, Serialize};
use serde_json::{json, Map};

use crate::*;

#[wasm_bindgen]
impl PianoGlobal {
    pub fn save(&self) -> String {
        self.sheet.save()
    }

    pub fn save_comp(&self) -> String {
        self.sheet.save_comp()
    }

    pub fn load(&mut self, json: String) {
        self.sheet.load(json);
    }

    pub fn add_track(&mut self, inst: i32, colo_s: String) {
        if inst < 0 {
            return;
        }
        self.sheet.add_inst(inst as usize, colo_s);
        //self.tracks.push(Track {
        //    inst: inst as usize,
        //    hide: false,
        //    colo: JsValue::from(colo_s.as_str()),
        //    colo_s,
        //    notes: Default::default(),
        //});
        //self.resize(-1);
    }

    pub fn resize(&mut self, n: i32) {
        let n = if n < 0 { 0 } else { n as usize };

        self.sheet.resize(n);
        let l = self.sheet.time() + 4;
        let h = self.sheet.tr_len() + 25;

        // let tar = max_note.max(n);
        // self.maxnote = tar;

        // self.shunk();

        // self.tracks
        //     .iter_mut()
        //     .for_each(|t| t.resize(tar, Default::default()));

        let width = self.cctx.cube_w * l as f64;
        let height = self.cctx.cube_h * h as f64;

        self.cctx.resize(l, h);

        self.draw_all();
        //alert("asdfasdf");
    }

    pub fn load_midi(&mut self, midi: &mut [u8]) {
        if let Ok(mi) = midly::Smf::parse(midi) {
            crate::alert(format!("--{}--", mi.tracks.len()).as_str());
        } else {
            crate::alert("failed");
        }
    }
}

impl PianoGlobal {
    fn shunk(&mut self) {
        //self.sheet.shunk();
        //let mut last = self.maxnote - 1;
    }
}

//#[derive(Serialize, Deserialize)]
//struct SaveConf {
//    tracks: Vec<Track>,
//
//    #[serde(default)]
//    version: usize,
//}
const NOTE: &[u8; 25] = b"ABCDEFGHIJKLMNOPQRSTUVWXY";
const BEAT: &[u8; 16] = b"0123456789abcdef";
