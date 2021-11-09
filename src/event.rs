use crate::PianoGlobal;

use wasm_bindgen::prelude::*;

use crate::map::*;

#[wasm_bindgen]
impl PianoGlobal {
    pub fn click(
        &mut self,
        x: i32,
        y: i32,
        down: bool,
        left: i32,
        _shift: bool,
        _ctrl: bool,
        _alt: bool,
    ) {
        let ylimit: i32 = self.tracks.len() as i32 * CELLH as i32;
        let xlimit: i32 = TITLW as i32;
        //log(format!("{}--{}--{}--{}--{}",xlimit,ylimit,x,y,left).as_str());
        //crate::l(format!("{}{}", left, down));
        match (down, (x > xlimit), (y > ylimit), left) {
            (true, true, true, 1) => self.click_edit(
                ((y - ylimit) / CELLH as i32) as u8,
                (x - xlimit) as usize / NOTEW as usize,
                true,
            ),
            (true, true, true, 2) => self.click_edit(
                ((y - ylimit) / CELLH as i32) as u8,
                (x - xlimit) as usize / NOTEW as usize,
                false,
            ),
            (true, true, false, 1) => self.click_del(
                (y / CELLH as i32) as u8,
                (x - xlimit) as usize / NOTEW as usize,
            ),
            (true, false, true, 1) => self.click_play(((y - ylimit) / CELLH as i32) as u8),
            (true, false, false, ..) => {}
            _ => {}
        }
        //log(format!("{}-{}-{}-{}-{}-{}-{}", x, y, down, left, shift, ctrl, alt).as_str());
    }
}

impl PianoGlobal {
    pub fn click_edit(&mut self, ic: u8, time: usize, on: bool) {
        let inote = time >> 2;
        let beat = 0b1000 >> (time & 0b11);
        let sel_track = self.rtd.sel_track;

        if let Some(i) = &self.insts.get(self.rtd.sel_inst as usize) {
            let ii = **i as u8;
            let mut changed = false;

            if let Some(Some(n)) = self.tracks.get_mut(sel_track).map(|t| t.get_mut(inote)) {
                if n.beat == 0 {
                    n.note = ic;
                }
                if n.note == ic {
                    if on {
                        //crate::l(format!("--------------------------{}-{}-{}", n.beat, beat,n & ));
                        if (n.beat & beat) == 0 {
                            changed = true
                        }
                        n.beat |= beat;
                    } else {
                        n.beat &= !beat;
                    }
                }
            }

            if changed {
                self.play(ii, 24 - ic);
            }

            self.draw_all();
        }
    }

    pub fn click_play(&mut self, ic: u8) {
        if let Some(i) = &self.insts.get(self.rtd.sel_inst as usize) {
            self.play(**i as u8, 24 - ic);
        }
        //if let Some(i) = &self.insts.get(ic as usize) {
        //    self.play(i.0, i.1);
        //}
    }

    pub fn click_del(&mut self, tc: u8, time: usize) {
        if let Some(n) = self.tracks[tc as usize].notes.get_mut(time >> 2) {
            //log(format!("{}--{}--{}--{}", tc, time, n.beat, !(0b1000 >> (time & 3))).as_str());
            n.beat &= !(0b1000 >> (time & 3));
            //log(format!("{}--{}--{}--{}", tc, time, n.beat, !(0b1000 >> (time & 3))).as_str());
            //self.flesh_insts();
            self.draw_all();
        }
    }
}
