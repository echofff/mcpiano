
use crate::*;
use wasm_bindgen::prelude::*;
use web_sys::{AudioBuffer, AudioContext, Request, RequestInit, Response};

#[derive(serde::Deserialize)]
pub struct NoteBox {
    pub name: String,
    pub hash: String,
    //size: usize,
    #[serde(skip_deserializing)]
    pub audio: Option<AudioBuffer>,
}

pub struct RuntimeData {
    pub sel_inst: usize,
    pub maxnote: usize,
    pub pause: bool,

    pub pos: (usize, usize),
    pub volumn: f32,

    // u8 is beat, in fromat 0bxxxx
    pub error_hl: Vec<(usize, u8)>,
    pub select_hl: Vec<(usize, u8)>,

    pub play_bt: usize,

    pub cube_w: f64,
    pub cube_h: f64,
    //pub cellw: f64,
    //pub cellh: f64,
    pub borde: f64,
    //pub titlw: f64,
    pub titles: usize,
    pub win_w: f64,
    pub win_h: f64,
}

pub struct Theme {
    pub control: [JsValue; 6],
    pub track_row: Vec<JsValue>,
    pub note_row: Vec<JsValue>,

    pub sel: JsValue,
    pub error: JsValue,
    pub play: JsValue,

    pub hover: JsValue,
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
            sel: "#2222cc4".into(),
            error: "#ff4444c4".into(),
            play: "#44cc44c4".into(),
            hover: "#66cccc88".into(),
        }
    }
}

impl RuntimeData {
    pub fn new() -> RuntimeData {
        RuntimeData {
            sel_inst: 11,
            maxnote: 20,
            pause: true,
            volumn: 0.7f32,
            pos: (0, 0),
            cube_w: 20f64,
            cube_h: 20f64,
            borde: 1f64,
            titles: 4,
            win_w: 1900f64,
            win_h: 1000f64,

            error_hl: Vec::new(),
            select_hl: Vec::new(),
            play_bt: 0,
        }
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
