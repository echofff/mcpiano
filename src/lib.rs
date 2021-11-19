mod utils;

mod draw;
mod event;
mod export;
mod map;
mod pianoglobal;
mod play;
mod saver;
mod sheet;
use draw::Draw;
use pianoglobal::*;

use wasm_bindgen::prelude::*;

use play::Player;

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
use crate::sheet::RedPianoV3;

#[wasm_bindgen]
impl PianoGlobal {
    pub async fn new() -> PianoGlobal {
        let actx = Player::new().await;

        let cctx = Draw::new();

        let theme = Theme::new();

        //let sheet = Box::new(RedPianoV2::new());
        let sheet = Box::new(RedPianoV3::new());

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

impl PianoGlobal {}
