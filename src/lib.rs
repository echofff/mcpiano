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
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;

use play::Player;

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
impl PianoGlobal {
    pub async fn new() -> PianoGlobal {
        let actx = Player::new().await;

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

        let theme = Theme::new();

        let sheet = Box::new(RedPianoV2::new());

        PianoGlobal {
            actx,
            cctx,
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
}
