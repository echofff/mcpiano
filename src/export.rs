use crate::map::*;
use crate::PianoGlobal;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl PianoGlobal {
    pub fn export(&mut self) -> String {
        self.sheet.export()
    }
}
