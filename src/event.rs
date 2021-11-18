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

        let cata = &[KeyCata::Down, KeyCata::Move, KeyCata::Up][cata as usize];
        let key = &[Key::Empty, Key::Left, Key::Right, Key::Empty, Key::Mid][key as usize];

        if self.sheet.click(Event {
            xi,
            yi,
            key,
            cata,
            area,
            shift,
            ctrl,
            alt,
        }) {
            self.actx.play(self.sheet.sel_inst, 23 - yi + ts);
        }
        self.draw_all();
    }

    pub fn input_key(&mut self, key: usize) -> bool {
        let (x, y) = self.pos;

        if let Some((inst, k)) = self.sheet.key(x, y, key) {
            self.draw_all();
            self.actx.play(inst, k);
            false
        } else {
            true
        }
    }
}
