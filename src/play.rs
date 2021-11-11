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

    pub fn play_start(&mut self) -> bool {
        self.rtd.pause ^= true;
        !self.rtd.pause
    }

    pub fn play_stage(&mut self) -> bool {
        let (ni, beat) = (self.rtd.play_bt >> 2, 0b1000 >> (self.rtd.play_bt & 0b11));

        if self.rtd.maxnote > ni {
            self.rtd.play_bt += 1;
            self.draw_all();
            self.tracks
                .iter()
                .filter_map(|t| t.get(ni).map(|n| (t.inst, n)))
                .for_each(|(inst, n)| {
                    if (n.beat & beat) != 0 {
                        self.play(inst as u8, n.note);
                    };
                });
            true
        } else {
            self.rtd.pause = true;
            self.rtd.play_bt = 0;
            self.draw_all();
            false
        }
    }
}
