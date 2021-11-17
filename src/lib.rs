mod utils;

mod draw;
mod event;
mod export;
mod map;
mod pianoglobal;
mod play;
mod saver;
mod sheet;
mod track;
use draw::Draw;
use pianoglobal::*;

use wasm_bindgen::prelude::*;
use wasm_bindgen::throw_str;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;
use web_sys::{AudioContext, Request, RequestInit, Response};

//use wee_alloc;
//#[cfg(feature = "wee_alloc")]
//#[global_allocator]
//

extern crate wee_alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[allow(unused_unsafe)]
pub fn l(s: String) {
    unsafe {
        log(s.as_str());
    }
}

use sheet::Sheet;

use crate::sheet::RedPianoV2;

#[wasm_bindgen]
pub struct PianoGlobal {
    actx: AudioContext,
    sounds: Vec<NoteBox>,

    //tracks: Vec<Track>,
    sheet: Box<dyn Sheet>,

    rt: RuntimeData,

    theme: Theme,

    cctx: Draw,
}

#[wasm_bindgen]
impl PianoGlobal {
    pub async fn new() -> PianoGlobal {
        let actx = AudioContext::new().unwrap_throw();
        //let mut notes: Vec<NoteBox> = conf.into_serde().unwrap_throw();
        let sounds = Self::create_soundbox(&actx).await;

        let (canv, cctx) = Self::scanvas();

        let cctx = Draw {
            cctx,
            canv,
            cube_w: 20f64,
            cube_h: 20f64,
            borde: 1f64,
            titles: 4,
            win_w: 1900f64,
            win_h: 1000f64,
        };

        //let tracks = vec![Track::new()];

        let theme = Theme::new();

        let sheet = Box::new(RedPianoV2::new());

        PianoGlobal {
            actx,
            cctx,
            sounds,
            sheet,

            //tracks,
            rt: RuntimeData::new(),
            theme,
        }
    }
}

impl PianoGlobal {
    fn scanvas() -> (HtmlCanvasElement, CanvasRenderingContext2d) {
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

        (canvas, context)
    }
    async fn create_soundbox(actx: &AudioContext) -> Vec<NoteBox> {
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
