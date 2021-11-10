use crate::PianoGlobal;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl PianoGlobal {
    pub fn play(&self, inst: u8, note: u8) {
        let a = self.actx.create_buffer_source().unwrap_throw();
        a.set_buffer(self.sounds[inst as usize].audio.as_ref());
        a.detune().set_value((note as f32 - 12f32) * 100f32);
        a.connect_with_audio_node(&self.actx.destination())
            .expect_throw("connect play");
        a.start().expect_throw("start play");
    }
    pub fn get_pause(&mut self) -> bool {
        self.rtd.pause
    }
    pub fn set_pause(&mut self, p: bool) {
        self.rtd.pause = p;
    }
    pub fn play_stage(&mut self, i: usize) {
        let ni = i >> 2;
        let bi = i & 3;

        if self.rtd.maxnote * 4 < i {
            self.rtd.pause = true;
        }

        self.tracks.iter().for_each(|t| {
            if let Some(n) = t.notes.get(ni) {
                if (n.beat & (0b1000 >> bi)) != 0 {
                    self.play(t.inst as u8, n.note)
                };
            };
        });
    }
}
