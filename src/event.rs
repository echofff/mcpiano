use crate::PianoGlobal;

use crate::draw::Area;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl PianoGlobal {
    pub fn input(
        &mut self,
        x: usize,
        y: usize,
        cata: i32,
        key: i32,
        _shift: bool,
        ctrl: bool,
        _alt: bool,
    ) {
        let (xi, yi) = (x / self.rtd.notew as usize, y / self.rtd.cellh as usize);
        let ts = self.tracks.len();
        let (area, xi, yi) = match (xi >= 4, yi >= ts) {
            (true, true) => (Area::EditPlane, xi - 4, yi - ts),
            (true, false) => (Area::TrackSecquence, xi - 4, yi),
            (false, true) => (Area::InstTitle, xi, yi - ts),
            (false, false) => (Area::TrackControl, xi, yi),
        };

        let (ni, beat) = (xi >> 2, 0b1000 >> (xi & 0b11) as u8);

        match (area, cata, key) {
            (Area::EditPlane, 0 | 1, 1) => self.click_edit(ni, beat, yi, ctrl),
            (Area::EditPlane, 0 | 1, 2) => self.click_del(ni, beat, yi, ctrl),
            (Area::TrackSecquence, 0 | 1, 1) => self.click_switch(yi),
            (Area::InstTitle, 0, 1) => self.click_play(yi as u8),
            (Area::TrackControl, 0, 1) => self.click_control(xi & 0b11, yi),
            _ => {}
        }
    }
}

impl PianoGlobal {
    fn click_control(&mut self, f: usize, i: usize) {
        match f {
            0 => {
                self.tracks.get_mut(i as usize).map(|n| n.hide = !n.hide);
                self.draw_all();
            }
            1 => {
                if let Some(true) = self
                    .tracks
                    .get(i as usize)
                    .map(|t| t.deleteable() && t.len() > 1)
                {
                    self.tracks.remove(i as usize);
                    self.resize(-1);
                }
            }
            _ => {
                self.rtd.sel_inst = self.tracks[i].inst;
                self.draw_all();
            }
        }
    }
    fn click_switch(&mut self, i: usize) {
        self.rtd.sel_inst = self.tracks[i].inst;
        self.draw_all();
    }

    fn click_edit(&mut self, ni: usize, beat: u8, y: usize, shift: bool) {
        let select = self.rtd.sel_inst;

        if let Some(n) = self
            .tracks
            .iter_mut()
            .filter(|t| t.inst == select)
            .filter_map(|t| t.get_mut(ni))
            .find(|n| n.note == 24 - y as u8)
        {
            if n.beat & beat == 0 {
                if !shift {
                    n.beat |= beat;
                } else {
                    n.beat = 0b1111;
                }
                self.draw_all();
                self.play(select as u8, 24 - y as u8);
            }
        } else if let Some(n) = self
            .tracks
            .iter_mut()
            .filter(|t| t.inst == select)
            .filter_map(|t| t.get_mut(ni))
            .find(|n| n.beat == 0)
        {
            n.note = 24 - y as u8;
            if n.beat & beat == 0 {
                if !shift {
                    n.beat |= beat;
                } else {
                    n.beat = 0b1111;
                }
                self.draw_all();
                self.play(select as u8, 24 - y as u8);
            }
        }
    }

    fn click_play(&mut self, ic: u8) {
        self.play(self.rtd.sel_inst as u8, 24 - ic);
    }

    fn click_del(&mut self, ni: usize, beat: u8, y: usize, shift: bool) {
        let mut change = false;
        self.tracks
            .iter_mut()
            .filter_map(|t| t.get_mut(ni))
            .filter(|n| n.note == 24 - y as u8)
            .for_each(|n| {
                if shift {
                    n.beat = 0;
                    change = n.beat != 0;
                } else {
                    n.beat &= !beat;
                    change = (n.beat & beat) == beat;
                }
            });
        if change {
            self.draw_all();
        }
    }
}
