mod utils;

mod draw;
mod event;
mod export;
mod map;
mod pianoglobal;
mod play;
mod saver;
mod track;
use pianoglobal::*;

use track::*;

use wasm_bindgen::prelude::*;
use wasm_bindgen::throw_str;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;
use web_sys::{AudioContext, Request, RequestInit, Response};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn l(s: String) {
    unsafe {
        log(s.as_str());
    }
}

#[wasm_bindgen]
pub struct PianoGlobal {
    actx: AudioContext,
    cctx: CanvasRenderingContext2d,
    canv: HtmlCanvasElement,
    sounds: Vec<NoteBox>,

    config: PianoConfig,
    rtd: RuntimeData,
}

#[wasm_bindgen]
impl PianoGlobal {
    pub async fn new() -> PianoGlobal {
        let actx = AudioContext::new().unwrap_throw();
        //let mut notes: Vec<NoteBox> = conf.into_serde().unwrap_throw();
        let sounds = Self::create_soundbox(&actx).await;

        let (canv, cctx) = Self::scanvas();

        let config = PianoConfig::new();

        let mut p = PianoGlobal {
            actx,
            cctx,
            canv,
            sounds,

            config,
            rtd: RuntimeData::new(),
        };

        p.resize(20);
        p
    }
}

impl PianoGlobal {
    pub async fn create_soundbox(actx: &AudioContext) -> Vec<NoteBox> {
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
            throw_str(format!("{:?}", e).as_str());
        } else {
            json.unwrap()
        };

        // Use serde to parse the JSON into a struct.
        let mut sounds: Vec<NoteBox> = json.into_serde().unwrap();

        for ele in &mut sounds {
            //ele.genab(&ctx).await.expect_throw("run-> for");
            let a = ele.genab(&actx).await;
            if let Err(a) = a {
                throw_str(format!("---------{:?}", a).as_str());
            }
        }
        sounds
    }
}
