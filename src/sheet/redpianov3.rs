use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use wasm_bindgen::{throw_str, JsValue, UnwrapThrowExt};

use crate::draw::{Area, Draw};
use crate::event::{Event, Key, KeyCata};
use crate::map::{DELAY_NAME, TIME_MARK};
use crate::map::{SYMBOL, SYMBOL_NAME};
use crate::mccommand::{items2String, Item};
use crate::play::KEYM;

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
    clips: Vec<Clip>,
    len: usize,

    cd: CommonData,
    //tmp: Vec<(usize, usize, usize)>,
}

struct Clip {
    note: usize,
    start: usize,
    end: usize,
}

//#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
//struct NoteEvent {
//    note: usize,
//    time: usize,
//    down: bool,
//    //down: usize,
//    //up: usize,
//}

impl RedPianoV3 {
    pub fn new() -> Self {
        RedPianoV3 {
            clips: Vec::new(),
            //start: vec![NoteDe::new()],
            //end: vec![NoteDe::new()],
            len: 4,
            cd: CommonData {
                sel_inst: 11,
                tpm: 2,
                error: Vec::new(),
            },
            //tmp: Vec::new(),
        }
    }

    //fn get<'a>(&'a  self,time:usize,note:usize) -> Option<&'a  NoteDe>{
    //    None
    //}
    //fn get_mut<'a>(&'a mut self,time:usize,note:usize) -> Option<&'a mut NoteDe>{
    //    None
    //}

    //fn gen_tmp(&mut self) {
    //    self.tmp.clear();
    //    let mut t = vec![0usize; 25];

    //    for e in self.events.iter() {
    //        if e.down {
    //            t[e.note] = e.time;
    //        } else {
    //            self.tmp.push((e.note, t[e.note], e.time));
    //        };
    //    }
    //}
    fn click_edit(&mut self, xi: usize, yi: usize) -> bool {
        let n = 24 - yi;
        self.error = Vec::new();
        //let ni = xi >> 2;
        //let len = self.notes.len();
        self.resize(yi + 1);
        //crate::alert(format!("--click------").as_str());
        //

        // if is in side a exist
        if let Some(c) = self
            .clips
            .iter()
            .find(|c| c.note == n && c.start <= xi && xi < c.end)
        {
            return false;
        }

        // get a clip start after
        let after = self
            .clips
            .iter_mut()
            .enumerate()
            .find(|(i, c)| c.note == n && xi <= c.start);

        // get a clip end before
        let before = self
            .clips
            .iter_mut()
            .enumerate()
            .filter(|(i, c)| c.note == n && c.end < xi)
            .last();

        // + + + + + + +
        // + - - - - - +

        match (before, after) {
            (Some((bi, bc)), Some((ai, ac))) if ac.start - bc.end < 4 => {
                //self.events[bi].up = self.events[ai].up;
                bc.end = ac.end;
                self.clips.remove(ai);
            }
            (Some((bi, bc)), _) if xi - bc.end < 2 => {
                bc.end = xi + 1;
            }
            (_, Some((ai, ac))) if ac.start - xi < 2 => {
                ac.start = xi;
            }
            _ => {}
        };

        //match (before, after) {
        //    (Some((bi, b)), Some((ai, a))) if a + b < 6 => {
        //        //self.events[bi].up = self.events[ai].up;
        //        self.events.remove(ai);
        //        self.events.remove(bi);
        //    }
        //    (_, Some((ai, a))) if a < 2 => {
        //        self.events[ai].time = xi;
        //    }
        //    (Some((bi, b)), _) if b < 2 => {
        //        self.events[bi].time = xi;
        //    }
        //    _ => {
        //        let i = self
        //            .events
        //            .iter()
        //            .enumerate()
        //            .rev()
        //            .find(|(_, e)| e.time < xi)
        //            .map(|(i, _)| i + 1)
        //            .unwrap_or(0usize);
        //        self.events.insert(
        //            i,
        //            NoteEvent {
        //                note: 24 - yi,
        //                time: xi,
        //                down: true,
        //            },
        //        );
        //        let i = self
        //            .events
        //            .iter()
        //            .enumerate()
        //            .rev()
        //            .find(|(_, e)| e.time < xi + 4)
        //            .map(|(i, _)| i + 1)
        //            .unwrap_or(0usize);
        //        self.events.insert(
        //            i,
        //            NoteEvent {
        //                note: 24 - yi,
        //                time: xi + 2,
        //                down: false,
        //            },
        //        );
        //    }
        //};

        //self.gen_tmp();

        //crate::alert(format!("--{:?}--",self.tmp).as_str());
        //crate::log(format!("--{:?}--", self.events).as_str());

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
        true
    }
    //fn click_del(&mut self, xi: usize, yi: usize) -> bool {
    //    if let Some((note, start, end)) = self
    //        .tmp
    //        .iter()
    //        .find(|(note, start, end)| *note == 24 - yi && *start <= xi && xi < *end)
    //    {
    //        if end - start <= 2 {
    //            self.events.remove(
    //                self.events
    //                    .iter()
    //                    .enumerate()
    //                    .find(|(_, e)| e.note == *note && e.time == *start && e.down)
    //                    .unwrap()
    //                    .0,
    //            );
    //            self.events.remove(
    //                self.events
    //                    .iter()
    //                    .enumerate()
    //                    .find(|(_, e)| e.note == *note && e.time == *end && !e.down)
    //                    .unwrap()
    //                    .0,
    //            );
    //        } else {
    //            if let Some(e) = self
    //                .events
    //                .iter_mut()
    //                .find(|e| e.time == xi && e.note == 24 - yi && e.down)
    //            {
    //                e.time += 1
    //            } else if let Some(e) = self
    //                .events
    //                .iter_mut()
    //                .find(|e| e.time == xi + 1 && e.note == 24 - yi && !e.down)
    //            {
    //                e.time -= 1
    //            }
    //        }
    //    }
    //    self.gen_tmp();
    //    true
    //}
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
                cata: KeyCata::Down | KeyCata::Move,
                key: Key::Left,
                xi,
                yi,
                ctrl,
                ..
            } => self.click_edit(xi, yi),
            //Event {
            //    area: Area::EditPlane,
            //    cata: KeyCata::Down | KeyCata::Move,
            //    key: Key::Right,
            //    xi,
            //    yi,
            //    ..
            //} => self.click_del(xi, yi),
            _ => false,
        }
    }

    fn draw(&self, c: &Draw) {
        c.style_fill("#338888");
        for Clip { note, start, end } in self.clips.iter() {
            c.rect(start + 4, 24 - note + 1, end - start, 1, true);
        }
    }

    fn save(&self) -> String {
        String::new()
        //serde_json::to_string(&json!({
        //    "version": 3,
        //    "events": self.events
        //}))
        //.unwrap()
    }
    fn load(&mut self, str: String) {
        //let SaverV3 { events, version } = serde_json::from_str(&str).unwrap();
        //self.events = events.unwrap();
        //self.gen_tmp();
    }

    fn save_comp(&self) -> String {
        String::new()
    }

    fn add_inst(&mut self, inst: usize, color_s: String) {
        //todo!()
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
        if let Some((i, _)) = KEYM.into_iter().enumerate().find(|(_, n)| *n == key) {
            if x > 4 {
                self.click_edit(x - 4, 24 - i);
            }
            return Some((self.sel_inst, i));
        }
        None
    }

    fn export(&mut self) -> String {
        String::new()
        //let mut items = [[Vec::new(), Vec::new()], [Vec::new(), Vec::new()]];
        //let mut time = 0;
        //let mut event = self.events.iter().filter(|e| e.down);
        //while let Some(e) = event.next() {
        //    if e.time > time {
        //        while e.time - time >= 16 {
        //            items[0][0].add(0, 25);
        //            items[0][1].add(1, 0);
        //            time += 4;
        //        }
        //        items[0][0].add(0, e.note);
        //        items[0][1].add(1, e.time - time);
        //        time += 4;
        //    } else {
        //        crate::alert(format!("Confident at time {}", time).as_str());
        //        self.error = vec![time];
        //        return String::new();
        //    }
        //}

        //let mut time = 0;
        //let mut event = self.events.iter().filter(|e| !e.down);
        //while let Some(e) = event.next() {
        //    if e.time > time {
        //        while e.time - time >= 16 {
        //            items[1][0].add(0, 25);
        //            items[1][1].add(1, 0);
        //            time += 4;
        //        }
        //        items[1][0].add(0, e.note);
        //        items[1][1].add(1, e.time - time);
        //        time += 4;
        //    } else {
        //        crate::alert(format!("Confident at time {}", time).as_str());
        //        self.error = vec![time];
        //        return String::new();
        //    }
        //}
        //crate::log(format!("-- {:?}", items).as_str());

        //format!("?????? ??????\r\n/give @p minecraft:chest{}\r\n?????? ??????\r\n/give @p minecraft:chest{}\r\n?????? ??????\r\n/give @p minecraft:chest{}\r\n?????? ??????\r\n/give @p minecraft:chest{}\r\n",
        //        items2String(&mut items[0][0],String::from("start_note")),
        //        items2String(&mut items[0][1],String::from("start_note")),
        //        items2String(&mut items[1][0],String::from("start_note")),
        //        items2String(&mut items[1][1],String::from("start_note")),
        //        )
    }

    fn play(&self, t: usize) -> Vec<(usize, usize)> {
        //self.tmp
        //    .iter()
        //    .filter(|(_, start, end)| *start + 1 <= t && t < *end + 1)
        //    .map(|(n, _, _)| (11, *n))
        //    .collect::<Vec<_>>()
    }
}

trait AutoItems {
    fn add(&mut self, t: usize, id: usize);
}
impl AutoItems for Vec<Item> {
    fn add(&mut self, t: usize, id: usize) {
        let (id, name) = if t == 0 {
            (SYMBOL[id], SYMBOL_NAME[id])
        } else {
            (TIME_MARK[id], DELAY_NAME[id])
        };
        if let Some(i) = self.last_mut() {
            if i.id == id {
                i.Count += 1;
                return;
            }
        }
        self.push(Item {
            id,
            tag: json!({ "display": { "Name":format!( "{{\"text\": \"{}\"}}",name ) } }),
            Slot: 0,
            Count: 1,
        })
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
//#[derive(serde::Deserialize)]
//struct SaverV3 {
//    events: Option<Vec<NoteEvent>>,
//    version: Option<i32>,
//}
