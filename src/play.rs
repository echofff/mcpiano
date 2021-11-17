use crate::PianoGlobal;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl PianoGlobal {
    pub fn set_volumn(&mut self, volumn: f32) {
        self.volumn = volumn;
    }
    pub fn play(&self, inst: u8, note: u8) {
        let a = self.actx.create_buffer_source().unwrap_throw();
        let g = self.actx.create_gain().unwrap_throw();

        a.set_buffer(self.sounds[inst as usize].audio.as_ref());
        a.detune().set_value((note as f32 - 12f32) * 100f32);
        g.gain().set_value(self.volumn);

        a.connect_with_audio_node(&g).expect_throw("connect play");
        g.connect_with_audio_node(&self.actx.destination())
            .expect_throw("connect play");

        a.start().expect_throw("start play");
    }

    pub fn play_start(&mut self) -> bool {
        self.pause ^= true;
        !self.pause
    }

    pub fn play_continue(&self) -> bool {
        self.sheet.time() / 4 > self.play_bt >> 2
    }

    pub fn play_stage(&mut self) -> bool {
        false
        //        let (ni, beat) = (self.play_bt >> 2, 0b1000 >> (self.play_bt & 0b11));
        //
        //        if self.maxnote > ni {
        //            self.play_bt += 1;
        //            self.draw_all();
        //            self.tracks
        //                .iter()
        //                .filter_map(|t| t.get(ni).map(|n| (t.inst, n)))
        //                .filter(|(_, n)| n.beat & beat != 0)
        //                .for_each(|(inst, n)| {
        //                    self.play(inst as u8, n.note);
        //                });
        //            true
        //        } else {
        //            self.pause = true;
        //            self.play_bt = 0;
        //            self.draw_all();
        //            false
        //        }
    }
}
