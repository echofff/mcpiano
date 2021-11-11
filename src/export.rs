use crate::map::*;
use crate::PianoGlobal;
use crate::Track;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl PianoGlobal {
    pub fn export(&self) -> String {
        let mut res = String::new();

        self.tracks.iter().enumerate().for_each(|(i, t)| {
            let (n, b) = Self::track_pre(t);

            res.push_str(Self::shulk_i(&n, i, true).as_str());
            res.push_str(Self::shulk_i(&b, i, false).as_str());
        });
        res
    }
}
impl PianoGlobal {
    fn shulk_i(n: &Vec<Vec<(u8, usize)>>, i: usize, note: bool) -> String {
        let typ = if note { "音符" } else { "节奏" };

        let shulkstart = vec![ format!( "\n\r\n\r第{}条{}\n\r/give @p minecraft:chest{{\"display\":{{\"Name\":\"{{\\\"text\\\":\\\"音轨#{}  {}序列\\\"}}\"}},\"BlockEntityTag\":{{\"Items\":[",i+1,typ,i+1,typ)].into_iter();
        let shulkend = vec![String::from("]}}")].into_iter();

        let shulkbody = n
            .iter()
            .enumerate()
            .flat_map(|(si,sbox)| {
                let b = sbox.iter().enumerate().map(|(i, n)| {
                    let (obj,name) = if note {
                        ( SYMBOL[n.0 as usize], TITLE[n.0 as usize])
                    }else{
                        ( TIME_MARK[n.0 as usize], TIME_MARK_NAME[n.0 as usize])
                    };
                    format!( "{{\"id\":\"{}\",\"tag\":{{\"display\":{{\"Name\":\"{{\\\"text\\\": \\\"{}\\\"}}\"}}}},\"Slot\":{},\"Count\":{}}},", obj,name, i, n.1)
                }) ;

                let start = vec![String::from(
                    "{\"id\":\"minecraft:shulker_box\",\"tag\":{\"BlockEntityTag\":{\"Items\":[",
                )]
                .into_iter();

                let end=vec![format!("]}}}},\"Slot\":{},\"Count\":1}},",si)].into_iter();

                start.chain(b).chain(end)
            });

        shulkstart
            .chain(shulkbody)
            .chain(shulkend)
            .collect::<Vec<_>>()
            .join("")
    }

    fn track_pre(t: &Track) -> (Vec<Vec<(u8, usize)>>, Vec<Vec<(u8, usize)>>) {
        let mut org = t.notes.iter().map(|n| {
            if n.beat == 0 {
                (25, 0)
            } else {
                (n.note, n.beat)
            }
        });

        let curr = org.next().unwrap();
        let mut cur_note = (curr.0, 1usize);
        let mut cur_beat = (curr.1, 1usize);

        let mut notes = Vec::new();
        let mut beats = Vec::new();

        let mut shulk_note = Vec::new();
        let mut shulk_beat = Vec::new();

        while let Some(next) = org.next() {
            if next.0 == cur_note.0 {
                cur_note.1 += 1;
            } else {
                if notes.len() == 27 {
                    shulk_note.push(notes);
                    notes = Vec::new();
                }
                notes.push(cur_note);
                cur_note = (next.0, 1);
            }

            if next.1 == cur_beat.0 {
                cur_beat.1 += 1;
                //crate::log(&"beat add");
            } else {
                if beats.len() == 27 {
                    shulk_beat.push(beats);
                    beats = Vec::new();
                }
                beats.push(cur_beat);
                cur_beat = (next.1, 1);
            }
        }

        while notes.len() < 27 {
            notes.push((26 + (notes.len() % 2) as u8, 1));
        }
        while beats.len() < 27 {
            beats.push((16 + (notes.len() % 2) as u8, 1));
        }

        shulk_note.push(notes);
        shulk_beat.push(beats);

        (shulk_note, shulk_beat)
    }
}
