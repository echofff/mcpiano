use std::ops::Deref;
use std::ops::DerefMut;

use crate::*;
use wasm_bindgen::prelude::*;
use web_sys::{AudioBuffer, AudioContext, Request, RequestInit, Response};

use crate::track::*;

#[derive(serde::Deserialize)]
pub struct NoteBox {
    pub name: String,
    pub hash: String,
    //size: usize,
    #[serde(skip_deserializing)]
    pub audio: Option<AudioBuffer>,
}

#[derive( Clone, serde::Deserialize, serde::Serialize)]
pub struct PianoConfig {
    pub tracks: Vec<Track>,
    //pub insts: Vec<usize>,
}

pub struct RuntimeData {
    pub sel_inst: usize,
    pub sel_track: usize,
    pub maxnote: usize,
    pub pause: bool,

    pub notew: f64,
    pub cellw: f64,
    pub cellh: f64,
    pub borde: f64,
    pub titlw: f64,
    pub tablw: f64,
    pub tablh: f64,
}

pub struct Theme {
    pub control: [JsValue; 6],
    pub track_row: Vec<JsValue>,
    pub note_row: Vec<JsValue>,
}

impl Theme {
    pub fn new() -> Theme {
        Theme {
            control: [
                "green".into(),
                "blue".into(),
                "red".into(),
                "oraange".into(),
                "yellow".into(),
                "red".into(),
            ],
            track_row: vec!["#22443322".into(), "#44223333".into()],
            note_row: vec![
                "#aaaaaa".into(),
                "#ffffff".into(),
                "#aaaaaa".into(),
                "#ffffff".into(),
                "#aaaaaa".into(),
                "#ffffff".into(),
                "#ffffff".into(),
                "#aaaaaa".into(),
                "#ffffff".into(),
                "#aaaaaa".into(),
                "#ffffff".into(),
                "#ffffff".into(),
            ],
        }
    }
}

impl RuntimeData {
    pub fn new() -> RuntimeData {
        RuntimeData {
            sel_inst: 0,
            sel_track: 0,
            maxnote: 0,
            pause: false,
            notew: 20f64,
            cellw: 80f64,
            cellh: 20f64,
            borde: 1f64,
            titlw: 80f64,
            tablw: 1900f64,
            tablh: 1000f64,
        }
    }
}

impl PianoConfig {
    pub fn new() -> PianoConfig {
        let tracks = vec![Track::new()];
        //let insts = vec![11];

        PianoConfig { tracks }
    }
}

impl Deref for PianoGlobal {
    type Target = PianoConfig;

    fn deref(&self) -> &Self::Target {
        &self.config
    }
}

impl DerefMut for PianoGlobal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.config
    }
}

impl NoteBox {
    pub async fn genab(&mut self, ctx: &AudioContext) -> Result<(), JsValue> {
        let mut opts = RequestInit::new();
        opts.method("GET");
        let urls = format!("./res/{}", self.hash);

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
