use crate::PianoGlobal;

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

        match (area, down, left) {
            (Area::EditPlane, 0 | 1, _) => self.click_edit(ni, beat, yi, left),
            (Area::TrackSecquence, 0 | 1, 1) => self.click_switch(yi),
            (Area::InstTitle, 0, 1) => self.click_play(yi as u8),
            (Area::TrackControl, 0, 1) => self.click_control(xi & 0b11, yi),
            _ => {}
        }
    }
}

impl PianoGlobal {
    pub fn click_control(&mut self, f: usize, i: usize) {
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
                    }
                }
            }
            _ => {
                self.rtd.sel_inst = self.tracks[i].inst;
                self.draw_all();
            }
        }
    }
    pub fn click_switch(&mut self, i: usize) {
        self.rtd.sel_inst = self.tracks[i].inst;
        self.draw_all();
    }

    pub fn click_edit(&mut self, ni: usize, beat: u8, y: usize, key: i32) {
        let select = self.rtd.sel_inst;

        if let Some(Some(n)) = self
            .tracks
            .iter_mut()
            .filter(|t| t.inst == select)
            .map(|t| t.get_mut(ni))
            .find(|n| {
                if let Some(n) = n {
                    n.note == 24 - y as u8
                } else {
                    false
                }
            })
        {
            if key == 1 && n.beat & beat == 0 {
                n.beat |= beat;
                self.draw_all();
            } else if key == 2 && n.beat & !beat == 0 {
                n.beat ^= !beat;
                self.draw_all();
            }
        } else if let Some(Some(n)) = self
            .tracks
            .iter_mut()
            .filter(|t| t.inst == select)
            .map(|t| t.get_mut(ni))
            .find(|n| if let Some(n) = n { n.beat == 0 } else { false })
        {
            n.note = 24 - y as u8;
            if key == 1 && n.beat & beat == 0 {
                n.beat |= beat;
                self.draw_all();
            } else if key == 2 && n.beat & !beat == 0 {
                n.beat ^= !beat;
                self.draw_all();
            }
        }
    }


    pub fn click_play(&mut self, ic: u8) {
        self.play(self.rtd.sel_inst as u8, 24 - ic);
    }

    pub fn click_del(&mut self, tc: u8, time: usize) {
        if let Some(n) = self.tracks[tc as usize].notes.get_mut(time >> 2) {
            n.beat &= !(0b1000 >> (time & 3));
            self.draw_all();
        }
    }
}
