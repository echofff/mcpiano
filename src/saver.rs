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
    }

    pub fn resize(&mut self, n: i32) {
        let n = if n < 0 { 0 } else { n as usize };

        self.sheet.resize(n);
        let l = self.sheet.time() + 4;
        let h = self.sheet.tr_len() + 25;

        self.cctx.resize(l, h);

        self.draw_all();
    }

    pub fn load_midi(&mut self, midi: &mut [u8]) {
        if let Ok(mi) = midly::Smf::parse(midi) {
            crate::alert(format!("--{}--", mi.tracks.len()).as_str());
        } else {
            crate::alert("failed");
        }
    }
}
