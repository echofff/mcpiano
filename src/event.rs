use crate::PianoGlobal;

use crate::draw::Area;
use wasm_bindgen::prelude::*;

pub enum Key {
    Empty,
    Left,
    Right,
    Mid,
}

pub enum KeyCata {
    Down,
    Up,
    Move,
}

pub struct Event {
    pub xi: usize,
    pub yi: usize,

    pub key: &'static Key,
    pub cata: &'static KeyCata,
    pub area: &'static Area,

    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
}

#[wasm_bindgen]
impl PianoGlobal {
    pub fn input(
        &mut self,
        x: usize,
        y: usize,
        cata: i32,
        key: i32,
        shift: bool,
        ctrl: bool,
        alt: bool,
    ) {
        let (xi, yi) = (x / self.cctx.cube_w as usize, y / self.cctx.cube_h as usize);

        self.pos = if (xi, yi) == self.pos && cata == 1 {
            return;
        } else {
            (xi, yi)
        };

        let ts = self.sheet.tr_len();
        let (area, xi, yi) = match (xi >= 4, yi >= ts) {
            (true, true) => (&Area::EditPlane, xi - 4, yi - ts),
            (true, false) => (&Area::TrackSecquence, xi - 4, yi),
            (false, true) => (&Area::InstTitle, xi, yi - ts),
            (false, false) => (&Area::TrackControl, xi, yi),
        };

        //let (ni, beat) = (xi >> 2, 0b1000 >> (xi & 0b11) as u8);

        let cata = &[KeyCata::Down, KeyCata::Move, KeyCata::Up][cata as usize];
        let key = &[Key::Empty, Key::Left, Key::Right, Key::Empty, Key::Mid][key as usize];
        self.sheet.click(Event {
            xi,
            yi,

            key,
            cata,
            area,

            shift,
            ctrl,
            alt,
        });

        //match (area, cata, key) {
        //    (Area::EditPlane, 0 | 1, 1) => self.click_edit(ni, beat, yi, ctrl),
        //    (Area::EditPlane, 0 | 1, 2) => self.click_del(ni, beat, yi, ctrl),
        //    (Area::TrackSecquence, 0 | 1, 1) => self.click_switch(yi),
        //    (Area::InstTitle, 0, 1) => self.click_play(yi as u8),
        //    (Area::TrackControl, 0, 1) => self.click_control(xi & 0b11, yi),
        //    (Area::TrackSecquence, 0 | 1, 2) => self.click_time(xi),
        //    _ => {}
        //}
        self.draw_all();
    }

    pub fn input_key(&mut self, key: usize) -> bool {
        let (x, y) = self.pos;

        if !self.sheet.key(x, y, key) {
            self.draw_all();
            false
        } else {
            true
        }
    }
}

impl PianoGlobal {
    fn click_time(&mut self, t: usize) {
        self.play_bt = t;
    }
    fn click_control(&mut self, f: usize, i: usize) {

        //  self.sheet.click(i)
        //  match f {
        //      0 => {
        //          self.tracks.get_mut(i as usize).map(|t| t.hide = !t.hide);
        //          //self.draw_all();
        //      }
        //      1 => {
        //          if let Some(true) = self
        //              .tracks
        //              .get(i as usize)
        //              .map(|t| t.deleteable() && t.len() > 1)
        //          {
        //              self.tracks.remove(i as usize);
        //              self.resize(-1);
        //          }
        //      }
        //      _ => {
        //          self.sel_inst = self.tracks[i].inst;
        //          //self.draw_all();
        //      }
        //  }
    }
    fn click_switch(&mut self, i: usize) {
        //self.sel_inst = self.tracks[i].inst;
        //self.draw_all();
    }

    fn click_edit(&mut self, ni: usize, beat: u8, y: usize, shift: bool) {
        //let select = self.sel_inst;

        //  if let Some(n) = self
        //      .tracks
        //      .iter_mut()
        //      .filter(|t| t.inst == select)
        //      .filter_map(|t| t.get_mut(ni))
        //      .find(|n| n.note == 24 - y as u8)
        //  {
        //      if n.beat & beat == 0 {
        //          if !shift {
        //              n.beat |= beat;
        //          } else {
        //              n.beat = 0b1111;
        //          }
        //          //self.draw_all();
        //          self.play(select as u8, 24 - y as u8);
        //      }
        //  } else if let Some(n) = self
        //      .tracks
        //      .iter_mut()
        //      .filter(|t| t.inst == select)
        //      .filter_map(|t| t.get_mut(ni))
        //      .find(|n| n.beat == 0)
        //  {
        //      n.note = 24 - y as u8;
        //      if n.beat & beat == 0 {
        //          if !shift {
        //              n.beat |= beat;
        //          } else {
        //              n.beat = 0b1111;
        //          }
        //          //self.draw_all();
        //          self.play(select as u8, 24 - y as u8);
        //      }
        //  }
    }

    fn click_play(&mut self, ic: u8) {
        self.actx.play(self.sheet.sel_inst as u8, 24 - ic);
    }

    fn click_del(&mut self, ni: usize, beat: u8, y: usize, shift: bool) {
        //  let mut change = false;
        //  self.tracks
        //      .iter_mut()
        //      .filter_map(|t| t.get_mut(ni))
        //      .filter(|n| n.note == 24 - y as u8)
        //      .for_each(|n| {
        //          if shift {
        //              n.beat = 0;
        //              change = n.beat != 0;
        //          } else {
        //              n.beat &= !beat;
        //              change = (n.beat & beat) == beat;
        //          }
        //      });
        //  if change {
        //      //self.draw_all();
        //  }
    }
}

pub enum Action {
    Set,
    Reset,
    Delete,
}
