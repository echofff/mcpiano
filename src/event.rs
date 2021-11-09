use crate::{track::Track, PianoGlobal};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl PianoGlobal {
    pub fn click(
        &mut self,
        x: i32,
        y: i32,
        down: i32,
        left: i32,
        _shift: bool,
        _ctrl: bool,
        _alt: bool,
    ) {
        let ylimit: i32 = self.tracks.len() as i32 * self.rtd.cellh as i32;
        let xlimit: i32 = self.rtd.titlw as i32;
        //log(format!("{}--{}--{}--{}--{}",xlimit,ylimit,x,y,left).as_str());
        //crate::l(format!("{}{}", left, down));
        match (down, (x > xlimit), (y > ylimit), left) {
            (0 | 1, true, true, 1) => self.click_edit(
                ((y - ylimit) / self.rtd.cellh as i32) as u8,
                (x - xlimit) as usize / self.rtd.notew as usize,
                true,
            ),
            (0 | 1, true, true, 2) => self.click_edit(
                ((y - ylimit) / self.rtd.cellh as i32) as u8,
                (x - xlimit) as usize / self.rtd.notew as usize,
                false,
            ),
            (0, true, false, 1) => self.click_del(
                (y / self.rtd.cellh as i32) as u8,
                (x - xlimit) as usize / self.rtd.notew as usize,
            ),
            (0, false, true, 1) => self.click_play(((y - ylimit) / self.rtd.cellh as i32) as u8),
            (0, false, false, 1) => {
                let i = y / self.rtd.cellh as i32;
                let f = x / (self.rtd.titlw as i32 / 4);
                match f {
                    0 => {
                        self.tracks.get_mut(i as usize).map(|n| n.hide = !n.hide);
                        self.draw_all();
                    }
                    1 => {
                        if let Some(true) = self.tracks.get(i as usize).map(|t| t.deleteable()) {
                            if self.tracks.len() > 1 {
                                self.tracks.remove(i as usize);
                                self.resize(0);
                                self.rtd.sel_track = (i as usize - 1).max(0);
                            }
                        }
                    }
                    _ => {
                        self.rtd.sel_track = i as usize;
                        self.draw_all();
                    }
                }
            }
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

        if let Some(i) = &self.tracks.get(self.rtd.sel_track as usize) {
            let ii = i.inst as u8;
            let mut changed = false;

            if let Some(Track { hide: true, .. }) = self.tracks.get(sel_track) {
                return;
            }

            if let Some(Some(n)) = self.tracks.get_mut(sel_track).map(|t| t.get_mut(inote)) {
                if n.beat == 0 {
                    n.note = 24 - ic;
                }
                if n.note == 24 - ic {
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
        if let Some(i) = &self.tracks.get(self.rtd.sel_track as usize) {
            self.play(i.inst as u8, 24 - ic);
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
