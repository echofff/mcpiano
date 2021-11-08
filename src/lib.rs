mod utils;

mod draw;
mod event;
mod map;
mod play;

use wasm_bindgen::prelude::*;
use wasm_bindgen::throw_str;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::CanvasRenderingContext2d;
use web_sys::{AudioBuffer, AudioContext, Request, RequestInit, Response}; //// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
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
