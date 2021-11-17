use std::ops::{Deref, DerefMut};
use std::process::Command;

use wasm_bindgen::JsValue;

use crate::draw::{Area, Draw};
use crate::event::{self, Action, Event, Key, KeyCata};

use super::{CommonData, Sheet};

pub struct RedPianoV2 {
    tracks: Vec<Track>,
    cd: CommonData,
}

impl RedPianoV2 {
    pub fn new() -> RedPianoV2 {
        RedPianoV2 {
            tracks: vec![Track::new()],
            cd: CommonData {
                sel_inst: 11,
                tpm: 2,
            },
        }
    }
}

impl Sheet for RedPianoV2 {
    fn tr_len(&self) -> usize {
        self.tracks.len()
    }

    fn colo(&self, n: usize) -> usize {
        3usize
    }

    fn save(&self) -> String {
        String::from("TODO")
    }
    fn load(&mut self, str: String) {}

    fn save_comp(&self) -> String {
        String::from("TODO")
    }

    fn add_inst(&self, inst: usize, color_s: String) {}

    fn resize(&mut self, tar: usize) -> usize {
        let tar = self
            .tracks
            .iter()
            .map(|a| a.true_len())
            .max()
            .unwrap_or(0)
            .max(tar);

        self.tracks
            .iter_mut()
            .for_each(|t| t.resize(tar, Default::default()));

        self.tracks.len() * 4
    }

    //fn shunk(&mut self) {
    //    while self.tracks.iter().all(|t| {
    //        if let Some(Note { beat: 0, .. }) = t.get(last) {
    //            true && t.len() > self.maxnote
    //        } else {
    //            false
    //        }
    //    }) {
    //        self.tracks.iter_mut().for_each(|t| {
    //            t.pop();
    //        });
    //        last -= 1;
    //    }
    //}

    fn time(&self) -> usize {
        self.tracks.iter().map(|t| t.len()).max().unwrap_or(1) * 4
    }

    fn click(&mut self, event: crate::event::Event) {
        let (ni, beat) = (event.xi >> 2, 0b1000 >> (event.xi & 0b11) as u8);
        match event {
            Event {
                area: Area::EditPlane,
                cata: KeyCata::Down | KeyCata::Move,
                key: Key::Left,
                yi,
                ctrl,
                ..
            } => self.click_edit(ni, beat, yi, ctrl),
            _ => {}
        }
    }

    fn draw(&self, c: &Draw) {
        let l = self.tr_len();
        self.tracks
            .iter()
            .enumerate()
            .filter(|(_, t)| t.inst != self.sel_inst)
            .for_each(|(i, t)| t.draw_track(c, i, l));
        self.tracks
            .iter()
            .enumerate()
            .filter(|(_, t)| t.inst == self.sel_inst)
            .for_each(|(i, t)| t.draw_track(c, i, l));
    }
}

impl RedPianoV2 {
    fn click_time(&mut self, t: usize) {
        //self.play_bt = t;
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

    fn click_edit(&mut self, ni: usize, beat: u8, y: usize, ctrl: bool) {
        //let select = self.sel_inst;

        if let Some(n) = self
            .tracks
            .iter_mut()
            //.filter(|t| t.inst == select)
            .filter_map(|t| t.get_mut(ni))
            .find(|n| n.note == 24 - y as u8)
        {
            //crate::l(format!("--{}--{}--{}--{}--{}",ni,beat,y,shift,n.beat));
            if n.beat & beat == 0 {
                if !ctrl {
                    n.beat |= beat;
                } else {
                    n.beat = 0b1111;
                }
                //self.draw_all();
                //self.play(select as u8, 24 - y as u8);
            }
        } else if let Some(n) = self
            .tracks
            .iter_mut()
            //.filter(|t| t.inst == select)
            .filter_map(|t| t.get_mut(ni))
            .find(|n| n.beat == 0)
        {
            n.note = 24 - y as u8;
            if n.beat & beat == 0 {
                if !ctrl {
                    n.beat |= beat;
                } else {
                    n.beat = 0b1111;
                }
                //self.draw_all();
                //self.play(select as u8, 24 - y as u8);
            }
        }
    }

    fn click_play(&mut self, ic: u8) {
        //self.play(self.sel_inst as u8, 24 - ic);
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

#[derive(Clone, serde::Deserialize, serde::Serialize, Default)]
pub struct Note {
    pub note: u8,
    pub beat: u8,
}

#[derive(Clone, serde::Deserialize, serde::Serialize, Default)]
pub struct Track {
    pub inst: usize,
    pub hide: bool,

    #[serde(skip_deserializing, skip_serializing)]
    pub colo: JsValue,

    pub colo_s: String,

    pub notes: Vec<Note>,
}

impl Track {
    pub fn new() -> Track {
        let notes = vec![Default::default()];
        let inst = 11;
        let hide = false;
        let colo = JsValue::from_str("#9a9dea");
        let colo_s = String::from("#9a9dea");
        Track {
            inst,
            hide,
            notes,
            colo,
            colo_s,
        }
    }
    pub fn deleteable(&self) -> bool {
        self.iter().all(|n| n.beat == 0)
    }
    pub fn true_len(&self) -> usize {
        self.iter()
            .enumerate()
            .rev()
            .find(|(_, n)| n.beat != 0)
            .map(|(i, _)| i + 1)
            .unwrap_or(0)
    }
    pub fn gn(&self, ti: usize) -> Option<&Note> {
        self.get(ti >> 2)
    }
    pub fn gnb(&self, ti: usize) -> Option<(&Note, u8)> {
        self.get(ti >> 2).map(|n| (n, ti.to_beat()))
    }
    pub fn gnb_mut(&mut self, ti: usize) -> Option<(&mut Note, u8)> {
        self.get_mut(ti >> 2).map(|n| (n, ti.to_beat()))
    }
    pub fn gn_mut(&mut self, ti: usize) -> Option<&mut Note> {
        self.get_mut(ti >> 2)
    }

    fn draw_track(&self, c: &Draw, i: usize, l: usize) {
        //let theme = &self.theme;

        //draw control part
        //let is_selected = self.sel_inst == t.inst;

        //self.cctx.set_fill_style(&theme.control[t.hide as usize]);
        //self.draw_rect(0, ti, 1, 1, true);
        ////self.draw_beat(&area, 0, ti, 0b1000);

        //self.cctx.set_fill_style(&theme.control[2]);
        //self.draw_rect(1, ti, 1, 1, true);
        ////self.draw_beat(&area, 0, ti, 0b0100);

        //self.cctx
        //    .set_fill_style(&theme.control[3 + is_selected as usize]);
        //self.draw_rect(2, ti, 1, 1, true);
        ////self.draw_beat(&area, 0, ti, 0b0010);

        //self.cctx.set_fill_style(&t.colo);
        //self.draw_rect(3, ti, 1, 1, true);
        ////self.draw_beat(&area, 0, ti, 0b0001);

        self.notes
            .iter()
            .enumerate()
            .filter(|(_, n)| n.beat != 0)
            .for_each(|(ni, n)| {
                // draw under line for every note that is not empty.
                c.down_line(ni, i);

                n.draw_beat(c, ni * 4 + c.titles, i);

                if !self.hide {
                    n.draw_beat(c, ni * 4 + c.titles, 24 - n.note as usize + l);
                }
            });
    }
}

impl Note {
    pub fn draw_beat(&self, c: &Draw, x: usize, y: usize) {
        // use different offset for different areas.
        //let (x, y) = match area {
        //    Area::TrackControl => (x, y),
        //    Area::TrackSecquence => (x * 4 + 4, y),
        //    Area::EditPlane => (x * 4 + 4, y + self.sheet.tr_len()),
        //    Area::InstTitle => (x * 4, y + self.sheet.tr_len()),
        //};
        //let cv = &self.rtd;

        //let y = y as f64 * cv.cellh + cv.borde;
        //let w = cv.notew - cv.borde * 2f64;
        //let h = cv.cellh - cv.borde * 4f64;

        (x..x + 4)
            .into_iter()
            .zip([0b1000, 0b0100, 0b0010, 0b0001].into_iter())
            .filter(|(_, b)| b & self.beat != 0)
            .for_each(|(x, _)| {
                //let x = x as f64 * cv.notew + cv.borde;
                //self.cctx.fill_rect(x, y, w, h);
                c.rect(x, y, 1, 1, true);
            })
    }
}

trait BeatIndex {
    fn to_beat(&self) -> u8;
}

impl BeatIndex for usize {
    fn to_beat(&self) -> u8 {
        0b11 >> self & 0b11
    }
}

impl Deref for RedPianoV2 {
    type Target = CommonData;

    fn deref(&self) -> &Self::Target {
        &self.cd
    }
}
impl DerefMut for RedPianoV2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cd
    }
}

impl Deref for Track {
    type Target = Vec<Note>;

    fn deref(&self) -> &Self::Target {
        &self.notes
    }
}
impl DerefMut for Track {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.notes
    }
}
