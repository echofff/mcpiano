use std::ops::{Deref, DerefMut};

use serde_json::{json, Map, Value};
use wasm_bindgen::{throw_str, JsValue, UnwrapThrowExt};

use crate::draw::{Area, Draw};
use crate::event::{Event, Key, KeyCata};

use super::{CommonData, Sheet};

//#[derive(Copy, Clone, Debug)]
//struct NoteDe {
//    note: usize,
//    delay: usize,
//}
//
//impl NoteDe {
//    pub fn new() -> Self {
//        NoteDe { note: 25, delay: 0 }
//    }
//}
//
pub struct RedPianoV3 {
    //start: Vec<NoteDe>,
    //end: Vec<NoteDe>,
    events: Vec<NoteEvent>,
    len: usize,

    cd: CommonData,
    tmp: Vec<(usize, usize, usize)>,
}

#[derive(Copy, Clone, Debug)]
struct NoteEvent {
    note: usize,
    time: usize,
    down: bool,
    //down: usize,
    //up: usize,
}

impl NoteEvent {
    //fn contain(&self, x: usize) -> bool {
    //    x < self.up && x > self.down
    //}
}

impl RedPianoV3 {
    pub fn new() -> Self {
        RedPianoV3 {
            events: Vec::new(),
            //start: vec![NoteDe::new()],
            //end: vec![NoteDe::new()],
            len: 4,
            cd: CommonData {
                sel_inst: 11,
                tpm: 2,
            },

            tmp: Vec::new(),
        }
    }

    //fn get<'a>(&'a  self,time:usize,note:usize) -> Option<&'a  NoteDe>{
    //    None
    //}
    //fn get_mut<'a>(&'a mut self,time:usize,note:usize) -> Option<&'a mut NoteDe>{
    //    None
    //}
    fn gen_tmp(&mut self) {
        self.tmp.clear();
        let mut t = vec![0usize; 25];

        for e in self.events.iter() {
            if e.down {
                t[e.note] = e.time;
            } else {
                self.tmp.push((e.note, t[e.note], e.time));
            };
        }
    }
}

impl Sheet for RedPianoV3 {
    fn tr_len(&self) -> usize {
        //self.len

        1
    }
    fn click(&mut self, event: Event) -> bool {
        match event {
            Event {
                area: Area::EditPlane,
                cata: KeyCata::Down,
                key: Key::Left,
                xi,
                yi,
                ctrl,
                ..
            } => {
                //let ni = xi >> 2;
                //let len = self.notes.len();
                self.resize(yi + 1);
                //crate::alert(format!("--click------").as_str());

                if let Some(e) = self
                    .events
                    .iter()
                    .find(|e| e.time > xi && e.note == 24 - yi)
                {
                    if !e.down {
                        return false;
                    }
                }

                let after = self
                    .events
                    .iter()
                    .enumerate()
                    .find(|(_, e)| e.time > xi && e.down && e.note == 24 - yi)
                    .map(|(i, e)| (i, e.time - xi));

                let before = self
                    .events
                    .iter()
                    .enumerate()
                    .rev()
                    .find(|(_, e)| e.time < xi && !e.down && e.note == 24 - yi)
                    .map(|(i, e)| (i, xi - e.time));

                match (before, after) {
                    (Some((bi, b)), Some((ai, a))) if a + b < 12 => {
                        //self.events[bi].up = self.events[ai].up;
                        self.events.remove(ai);
                        self.events.remove(bi);
                    }
                    (_, Some((ai, a))) if a < 6 => {
                        self.events[ai].time = xi;
                    }
                    (Some((bi, b)), _) if b < 6 => {
                        self.events[bi].time = xi;
                    }
                    _ => {
                        let i = self
                            .events
                            .iter()
                            .enumerate()
                            .rev()
                            .find(|(_, e)| e.time < xi)
                            .map(|(i, _)| i+1)
                            .unwrap_or(0usize);
                        self.events.insert(
                            i,
                            NoteEvent {
                                note: 24 - yi,
                                time: xi,
                                down: true,
                            },
                        );
                        let i = self
                            .events
                            .iter()
                            .enumerate()
                            .rev()
                            .find(|(_, e)| e.time < xi + 4)
                            .map(|(i, _)| i+1)
                            .unwrap_or(0usize);
                        self.events.insert(
                            i,
                            NoteEvent {
                                note: 24 - yi,
                                time: xi + 4,
                                down: false,
                            },
                        );
                    }
                };

                self.gen_tmp();

                //crate::alert(format!("--{:?}--",self.tmp).as_str());

                //.filter(|(i, e)| e.note == 24 - yi && e.time < xi + 4 && e.time > xi - 4)
                //.collect::<Vec<_>>();

                //if let Some((i, n)) = self
                //    .notes
                //    .iter_mut()
                //    .rev()
                //    .skip(len - ni - 1)
                //    .take(4)
                //    //.inspect(|n| crate::alert(format!("--{:?}--", n).as_str()))
                //    .enumerate()
                //    .find(|(_, n)| n.note > 24)
                //{
                //    n.note = 24 - yi;
                //    n.delay = i * 4 + (xi & 0b11);
                //    //crate::alert(format!("--click{:?}--", n,).as_str());
                //}
            }
            _ => return false,
        }
        false
    }

    fn draw(&self, c: &Draw) {
        for (note, start, end) in self.tmp.iter() {
            c.rect(start + 4, 24 - note + 1, end - start, 1, true);
        }
    }

    fn save(&self) -> String {
        todo!()
    }
    fn load(&mut self, str: String) {
        todo!()
    }

    fn save_comp(&self) -> String {
        todo!()
    }

    fn add_inst(&mut self, inst: usize, color_s: String) {
        todo!()
    }

    fn resize(&mut self, tar: usize) -> usize {
        if self.len < tar {
            self.len = tar
        }

        //if self.start.len() < tar {
        //    self.start.resize(tar, NoteDe::new());
        //    //crate::alert(format!("--{}--", tar).as_str());
        //};
        //if self.end.len() < tar {
        //    self.end.resize(tar, NoteDe::new());
        //    //crate::alert(format!("--{}--", tar).as_str());
        //};
        tar
    }
    fn time(&self) -> usize {
        self.len
    }

    fn key(&mut self, x: usize, y: usize, key: usize) -> Option<(usize, usize)> {
        todo!()
    }

    fn export(&self) -> String {
        String::new()
    }
}

impl Deref for RedPianoV3 {
    type Target = CommonData;

    fn deref(&self) -> &Self::Target {
        &self.cd
    }
}
impl DerefMut for RedPianoV3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cd
    }
}
