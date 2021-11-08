mod utils;

use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::slice::SliceIndex;

use wasm_bindgen::prelude::*;
use wasm_bindgen::throw_str;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::CanvasRenderingContext2d;
use web_sys::{AudioBuffer, AudioBufferSourceNode, AudioContext, Request, RequestInit, Response};
//// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
//// allocator.
//#[cfg(feature = "wee_alloc")]
//#[global_allocator]
//static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
#[derive(serde::Deserialize)]
struct NoteBox {
    name: String,
    hash: String,
    size: usize,

    #[serde(skip_deserializing)]
    audio: Option<AudioBuffer>,
}

#[wasm_bindgen]
#[derive(Debug)]
struct Track {
    notes: Vec<Note>,
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
struct Note {
    insi: u8,
    inst: u8,
    note: u8,
    beat: u8,
}

#[wasm_bindgen]
pub struct PianoGlobal {
    actx: AudioContext,
    cctx: CanvasRenderingContext2d,
    notes: Vec<NoteBox>,
    tracks: Vec<Track>,
    insts: Vec<(u8, u8)>,

    tick: i32,
    maxnote: usize,
    pause: bool,
}

#[wasm_bindgen]
impl PianoGlobal {
    #[wasm_bindgen]
    pub async fn new() -> PianoGlobal {
        let actx = AudioContext::new().unwrap_throw();
        //let mut notes: Vec<NoteBox> = conf.into_serde().unwrap_throw();
        let tick = 50;
        let pause = false;

        let mut opts = RequestInit::new();
        opts.method("GET");

        let url = "conf.json";

        let request = Request::new_with_str_and_init(&url, &opts).expect_throw("req");

        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .expect_throw("resp_value");

        // `resp_value` is a `Response` object.
        assert!(resp_value.is_instance_of::<Response>());
        let resp: Response = resp_value.dyn_into().unwrap();

        // Convert this other `Promise` into a rust `Future`.
        let json = JsFuture::from(resp.json().expect_throw("run->json2")).await;
        //.expect_throw("run->json");
        let json = if let Err(e) = json {
            let a = format!("{:?}", e);
            unsafe {
                log(&a);
            };
            throw_str("ffffff")
        } else {
            json.unwrap()
        };

        // Use serde to parse the JSON into a struct.
        let mut notes: Vec<NoteBox> = json.into_serde().unwrap();

        for ele in &mut notes {
            //ele.genab(&ctx).await.expect_throw("run-> for");
            let a = ele.genab(&actx).await;
            if let Err(a) = a {
                let a = format!("---------{:?}", a);
                unsafe {
                    log(&a);
                }
            }
        }

        let mut insts = Vec::new();

        (0..25).for_each(|i| insts.push((11, 24 - i)));

        let cctx = Self::scanvas();

        let tracks = vec![
            Track {
                notes: vec![
                    Note {
                        insi: 0,
                        inst: 11,
                        note: 10,
                        beat: 0
                    };
                    23
                ],
            },
            Track {
                notes: vec![
                    Note {
                        insi: 0,
                        inst: 11,
                        note: 11,
                        beat: 0
                    };
                    23
                ],
            },
            Track {
                notes: vec![
                    Note {
                        insi: 0,
                        inst: 11,
                        note: 12,
                        beat: 0
                    };
                    23
                ],
            },
            Track {
                notes: vec![
                    Note {
                        insi: 0,
                        inst: 11,
                        note: 13,
                        beat: 0
                    };
                    23
                ],
            },
        ];

        //let tracks = vec![
        //    Track {
        //        notes: vec![
        //            Note {
        //                insi: 0,
        //                inst: 11,
        //                note: 10,
        //                beat: 0b0011,
        //            },
        //            Note {
        //                insi: 0,
        //                inst: 11,
        //                note: 11,
        //                beat: 0b1011,
        //            },
        //            Note {
        //                insi: 0,
        //                inst: 11,
        //                note: 11,
        //                beat: 0b1011,
        //            },
        //            Note {
        //                insi: 0,
        //                inst: 11,
        //                note: 12,
        //                beat: 0b0000,
        //            },
        //            Note {
        //                insi: 0,
        //                inst: 11,
        //                note: 14,
        //                beat: 0b0010,
        //            },
        //            Note {
        //                insi: 0,
        //                inst: 11,
        //                note: 14,
        //                beat: 0b0000,
        //            },
        //            Note {
        //                insi: 0,
        //                inst: 11,
        //                note: 14,
        //                beat: 0b0000,
        //            },
        //            Note {
        //                insi: 0,
        //                inst: 11,
        //                note: 14,
        //                beat: 0b0000,
        //            },
        //            Note {
        //                insi: 0,
        //                inst: 11,
        //                note: 14,
        //                beat: 0b0000,
        //            },
        //            Note {
        //                insi: 0,
        //                inst: 11,
        //                note: 14,
        //                beat: 0b0000,
        //            },
        //            Note {
        //                insi: 0,
        //                inst: 11,
        //                note: 14,
        //                beat: 0b0000,
        //            },
        //            Note {
        //                insi: 0,
        //                inst: 11,
        //                note: 14,
        //                beat: 0b0000,
        //            },
        //        ],
        //    },
        //    Track {
        //        notes: vec![
        //            Note {
        //                insi: 0,
        //                inst: 11,
        //                note: 16,
        //                beat: 0b1011,
        //            },
        //            Note {
        //                insi: 0,
        //                inst: 11,
        //                note: 15,
        //                beat: 0b0000,
        //            },
        //            Note {
        //                insi: 0,
        //                inst: 11,
        //                note: 14,
        //                beat: 0b1001,
        //            },
        //            Note {
        //                insi: 0,
        //                inst: 11,
        //                note: 13,
        //                beat: 0b1010,
        //            },
        //            Note {
        //                insi: 0,
        //                inst: 11,
        //                note: 12,
        //                beat: 0b0011,
        //            },
        //            Note {
        //                insi: 0,
        //                inst: 11,
        //                note: 14,
        //                beat: 0b0000,
        //            },
        //            Note {
        //                insi: 0,
        //                inst: 11,
        //                note: 14,
        //                beat: 0b0000,
        //            },
        //            Note {
        //                insi: 0,
        //                inst: 11,
        //                note: 14,
        //                beat: 0b0000,
        //            },
        //            Note {
        //                insi: 0,
        //                inst: 11,
        //                note: 14,
        //                beat: 0b0000,
        //            },
        //            Note {
        //                insi: 0,
        //                inst: 11,
        //                note: 14,
        //                beat: 0b0000,
        //            },
        //            Note {
        //                insi: 0,
        //                inst: 11,
        //                note: 14,
        //                beat: 0b0000,
        //            },
        //            Note {
        //                insi: 0,
        //                inst: 11,
        //                note: 14,
        //                beat: 0b0000,
        //            },
        //        ],
        //    },
        //];

        PianoGlobal {
            actx,
            cctx,
            notes,
            tracks,
            insts,

            maxnote: 24 * 4 * 4,

            tick,
            pause,
        }
    }
    pub fn play(&self, inst: u8, note: u8) {
        let a = self.actx.create_buffer_source().unwrap_throw();
        a.set_buffer(self.notes[inst as usize].audio.as_ref());
        a.detune().set_value((note as f32 - 12f32) * 100f32);
        a.connect_with_audio_node(&self.actx.destination())
            .expect_throw("connect play");
        a.start().expect_throw("start play");
    }
    pub fn get_pause(&mut self) -> bool {
        self.pause
    }
    pub fn set_pause(&mut self, p: bool) {
        self.pause = p;
    }
    pub fn play_stage(&mut self, i: usize) {
        let ni = i >> 2;
        let bi = i & 3;

        if self.maxnote < i {
            self.pause = true;
        }

        self.tracks.iter().for_each(|t| {
            if let Some(n) = t.notes.get(ni) {
                if (n.beat & (0b1000 >> bi)) != 0 {
                    self.play(n.inst, n.note)
                };
            };
        });
    }
    pub fn scanvas() -> CanvasRenderingContext2d {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        context

        //context.fill_rect(20f64, 20f64, 50f64, 50f64);
        //let a = JsString::from("green");
        //let a = JsString::from("#88888820");
        //context.set_fill_style(&a.into());
        //context.fill_rect(50f64, 50f64, 330f64, 330f64);
    }
}

const NOTEW: f64 = 20f64;
const CELLW: f64 = 80f64;
const CELLH: f64 = 20f64;
const BORDE: f64 = 1f64;
const TITLW: f64 = 60f64;

const TABLW: f64 = 1900f64;
const TABLH: f64 = 1000f64;

#[wasm_bindgen]
impl PianoGlobal {
    pub fn draw_all(&mut self) {
        self.cctx.clear_rect(0f64, 0f64, TABLW, TABLH);

        self.flesh_insts();
        self.draw_backline();
        self.draw_insts();
        self.draw_tracks();
    }

    pub fn flesh_insts(&mut self) {
        self.tracks.iter_mut().for_each(|t| {
            t.notes.iter_mut().for_each(|n| {
                if let Some(i) = self.insts.get(n.insi as usize) {
                    if *i == (n.inst, n.note) {
                        return;
                    }
                }

                if let Some((insi, _)) = self
                    .insts
                    .iter()
                    .enumerate()
                    .find(|e| *e.1 == (n.inst, n.note))
                {
                    //log("find");
                    n.borrow_mut().insi = insi as u8;
                }

                //n.borrow_mut().
            })
        })
    }

    pub fn draw_tracks(&self) {
        let c = &self.cctx;

        let row_styles = [&"#22443322".into(), &"#44223333".into()];
        //let col_styles = [&"#22448322".into(), &"#22883333".into()];
        let ins_styles = [
            &"#9a9dea".into(),
            &"#7bd4ca".into(),
            &"#e53e0f".into(),
            &"#18d579".into(),
            &"#8cea44".into(),
            &"#645122".into(),
        ];

        let mut yoffset = 0f64 + BORDE;
        self.tracks.iter().enumerate().for_each(|(i, t)| {
            let mut xoffset = TITLW + BORDE;
            // bg color
            c.set_fill_style(row_styles[i % 2]);
            c.fill_rect(xoffset, yoffset, TABLW - BORDE * 2f64, CELLH - BORDE * 2f64);

            t.notes.iter().enumerate().for_each(|(j, n)| {
                if n.beat == 0 {
                    xoffset += CELLW;
                    return;
                }

                // draw cell
                c.set_fill_style(ins_styles[2]);
                c.fill_rect(xoffset, yoffset + CELLH - 3f64, CELLW - BORDE * 2f64, 4f64);

                // draw note
                [0, 1, 2, 3].iter().fold(0b1000, |mask, _| {
                    if mask & n.beat != 0 {
                        //c.set_fill_style(col_styles[(j + 1) % 2]);
                        c.set_fill_style(ins_styles[i]);
                        c.fill_rect(xoffset, yoffset, NOTEW - BORDE * 2f64, NOTEW - BORDE * 2f64);
                        c.fill_rect(
                            xoffset,
                            (self.tracks.len() + n.insi as usize) as f64 * CELLH,
                            NOTEW - BORDE * 2f64,
                            NOTEW - BORDE * 2f64,
                        );
                    }
                    xoffset += NOTEW;
                    mask >> 1
                });
            });
            yoffset += CELLH;
        })
    }

    pub fn draw_insts(&self) {
        let row_styles = [&"#22443322".into(), &"#44223333".into()];
        let c = &self.cctx;
        let mut yoffset = self.tracks.len() as f64 * CELLH + BORDE;
        self.insts.iter().enumerate().for_each(|(i, inst)| {
            c.set_fill_style(&"black".into());
            c.fill_text(TITLE[inst.1 as usize], 10f64, yoffset + CELLH * 0.6f64)
                .unwrap_throw();
            c.set_fill_style(row_styles[i % 2]);
            c.fill_rect(BORDE, yoffset, TABLW - BORDE * 2f64, CELLH - BORDE * 2f64);
            yoffset += CELLH;
        })
    }

    pub fn draw_backline(&self) {
        let c = &self.cctx;
        let line = [&"#444444FF".into(), &"#00000044".into()];

        let mut x = TITLW;
        let mut i = 0;

        while x < TABLW {
            c.begin_path();
            c.set_stroke_style(line[if i % 4 == 0 { 0 } else { 1 }]);

            c.move_to(x, 0f64);
            c.line_to(x, TABLH);
            x += NOTEW;

            c.stroke();
            i += 1;
        }
    }
}

const TITLE: [&str; 25] = [
    "0_F#", "1_G", "2_G#", "3_A", "4_A#", "5_B", "6_C", "7_C#", "8_D", "9_D#", "10_E", "11_F",
    "12_F#", "13_G", "14_G#", "15_A", "16_A#", "17_B", "18_C", "19_C#", "20_D", "21_D#", "22_E",
    "23_F", "24_F#",
];

#[wasm_bindgen]
impl PianoGlobal {
    pub fn click(
        &mut self,
        x: i32,
        y: i32,
        down: bool,
        left: i32,
        shift: bool,
        ctrl: bool,
        alt: bool,
    ) {
        let ylimit: i32 = self.tracks.len() as i32 * CELLH as i32;
        let xlimit: i32 = TITLW as i32;
        //log(format!("{}--{}--{}--{}--{}",xlimit,ylimit,x,y,left).as_str());
        match (down, (x > xlimit), (y > ylimit)) {
            (true, true, true) => self.click_edit(
                ((y - ylimit) / CELLH as i32) as u8,
                (x - xlimit) as usize / NOTEW as usize,
            ),
            (true, true, false) => self.click_del(
                (y / CELLH as i32) as u8,
                (x - xlimit) as usize / NOTEW as usize,
            ),
            (true, false, true) => self.click_play(((y - ylimit) / CELLH as i32) as u8),
            (true, false, false) => {}
            _ => {}
        }
        //log(format!("{}-{}-{}-{}-{}-{}-{}", x, y, down, left, shift, ctrl, alt).as_str());
    }

    pub fn click_edit(&mut self, ic: u8, time: usize) {
        //log(format!("{}--{}", ic, time).as_str());
        let i = &self.insts[ic as usize];
        self.play(i.0, i.1);

        if let Some(t) = self.tracks.iter_mut().find(|t| {
            if let Some(n) = t.notes.get(time >> 2) {
                //log(format!("{}--{}--{}",n.insi,n.note).as_str());
                (n.inst, n.note) == *i
            } else {
                false
            }
        }) {
            //log("-------------------------get A");
            let n = &mut t.notes[time >> 2];
            n.beat ^= 0b1000 >> (time & 0b11);
        } else if let Some(t) = self.tracks.iter_mut().find(|t| {
            if let Some(n) = t.notes.get(time >> 2) {
                n.beat == 0
            } else {
                false
            }
        }) {
            //log("-------------------------get B");
            let n = &mut t.notes[time >> 2];
            n.insi = ic;
            n.inst = i.0;
            n.note = i.1;
            n.beat ^= 0b1000 >> (time & 0b11);
        } else {
            //log(format!("{:?}", i).as_str());
            //log("notfound");
        }

        self.flesh_insts();
        self.draw_all();
    }

    pub fn click_play(&mut self, ic: u8) {
        if let Some(i) = &self.insts.get(ic as usize) {
            self.play(i.0, i.1);
        }
    }

    pub fn click_del(&mut self, tc: u8, time: usize) {
        if let Some(n) = self.tracks[tc as usize].notes.get_mut(time >> 2) {
            //log(format!("{}--{}--{}--{}", tc, time, n.beat, !(0b1000 >> (time & 3))).as_str());
            n.beat &= !(0b1000 >> (time & 3));
            //log(format!("{}--{}--{}--{}", tc, time, n.beat, !(0b1000 >> (time & 3))).as_str());
            self.flesh_insts();
            self.draw_all();
        }
    }
}

#[wasm_bindgen]
impl PianoGlobal {
    pub fn out(&self) {
        //log(format!("{:?}", self.tracks).as_str());
    }
}
#[wasm_bindgen]
impl NoteBox {
    async fn genab(&mut self, ctx: &AudioContext) -> Result<(), JsValue> {
        let mut opts = RequestInit::new();
        opts.method("GET");
        let urls = format!("/res/{}", self.hash);

        let req = Request::new_with_str_and_init(&urls, &opts)?;
        let window = web_sys::window().unwrap();

        let resp: Response = JsFuture::from(window.fetch_with_request(&req))
            .await?
            .dyn_into()?;
        let ab = JsFuture::from(resp.array_buffer()?).await?.dyn_into()?;

        let ab = JsFuture::from(ctx.decode_audio_data(&ab)?)
            .await?
            .dyn_into()?;

        self.audio = Some(ab);

        Ok(())
    }
}

#[wasm_bindgen]
struct expotorv1{

}
