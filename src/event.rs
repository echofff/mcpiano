use crate::{track::Track, PianoGlobal};

use crate::draw::Area;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl PianoGlobal {
    pub fn click(
        &mut self,
        x: usize,
        y: usize,
        down: i32,
        left: i32,
        _shift: bool,
        _ctrl: bool,
        _alt: bool,
    ) {
        let ylimit = self.tracks.len() * self.rtd.cellh as usize;
        let xlimit = self.rtd.titlw as usize;

        let (area, x, y) = match (x > xlimit, y > ylimit) {
            (true, true) => (Area::EditPlane, x - xlimit, y - ylimit),
            (true, false) => (Area::TrackSecquence, x - xlimit, y),
            (false, true) => (Area::InstTitle, x, y - ylimit),
            (false, false) => (Area::TrackControl, x, y),
        };
        let (xi, yi) = (x / self.rtd.notew as usize, y / self.rtd.cellh as usize);

        let (ni, beat) = (xi >> 2, 0b1000 >> (xi & 0b11) as u8);

        //log(format!("{}--{}--{}--{}--{}",xlimit,ylimit,x,y,left).as_str());
        //crate::l(format!("{}{}", left, down));
        match (area, down, left) {
            (Area::EditPlane, 0 | 1, _) => self.click_edit(ni, beat, yi, left),
            (Area::TrackSecquence, 0 | 1, 1) => self.click_switch(yi),
            (Area::InstTitle, 0, 1) => self.click_play(yi as u8),
            (Area::TrackControl, 0, 1) => self.click_control(xi & 0b11, yi),
            _ => {}
        }
        //log(format!("{}-{}-{}-{}-{}-{}-{}", x, y, down, left, shift, ctrl, alt).as_str());
    }
}

impl PianoGlobal {
    pub fn click_control(&mut self, f: usize, i: usize) {
        match f {
            0 => {
                crate::alert("111");
                self.tracks.get_mut(i as usize).map(|n| n.hide = !n.hide);
                self.draw_all();
            }
            1 => {
                crate::alert("1122221");
                if let Some(true) = self.tracks.get(i as usize).map(|t| t.deleteable()) {
                    if self.tracks.len() > 1 {
                        self.tracks.remove(i as usize);
                        self.resize(0);
                        self.rtd.sel_track = (i as usize - 1).max(0);
                    }
                }
            }
            _ => {
                crate::alert("1133331");
                self.rtd.sel_track = i as usize;
                self.draw_all();
            }
        }
    }
    pub fn click_switch(&mut self, i: usize) {
        self.rtd.sel_track = i;
        self.draw_all();
    }

    pub fn click_edit(&mut self, ni: usize, beat: u8, y: usize, key: i32) {
        let sel_track = self.rtd.sel_track;

        if let Some(i) = &self.tracks.get(self.rtd.sel_track as usize) {
            let ii = i.inst as u8;
            let mut changed = false;

            if let Some(Track { hide: true, .. }) = self.tracks.get(sel_track) {
                return;
            }

            if let Some(Some(n)) = self.tracks.get_mut(sel_track).map(|t| t.get_mut(ni)) {
                if n.beat == 0 {
                    n.note = 24 - y as u8;
                }
                if n.note == 24 - y as u8 {
                    if key == 1 {
                        //crate::l(format!("--------------------------{}-{}-{}", n.beat, beat,n & ));
                        if (n.beat & beat) == 0 {
                            changed = true
                        }
                        n.beat |= beat;
                    } else if key == 2 {
                        n.beat &= !beat;
                    }
                }
            }

            if changed {
                self.play(ii, 24 - y as u8);
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
