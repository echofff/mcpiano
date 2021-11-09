use std::ops::Deref;
use std::ops::DerefMut;

use crate::*;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;
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

fn get_actx() -> AudioContext {
    AudioContext::new().unwrap_throw()
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct PianoConfig {
    pub tracks: Vec<Track>,
    pub insts: Vec<usize>,
}

#[derive(Debug, Default)]
pub struct RuntimeData {
    pub sel_inst: usize,
    pub sel_track: usize,
    pub maxnote: usize,
    pub pause: bool,
}

impl PianoConfig {
    pub fn new() -> PianoConfig {
        let tracks = vec![Track::new()];
        let insts = vec![11];

        PianoConfig { tracks, insts }
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
