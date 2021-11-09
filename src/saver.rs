use crate::*;

#[wasm_bindgen]
impl PianoGlobal {
    pub fn save(&self) -> String {
        match serde_json::to_string(&self.config) {
            Ok(s) => s,
            Err(e) => {
                format!("save failed {:?}", e)
            }
        }
    }

    pub fn load(&mut self, json: String) {
        //serde_json::to_string(&self.config).unwrap_throw()
        match serde_json::from_str(json.as_str()) {
            Ok(c) => {self.config = c;
                self.draw_all();
            },
            Err(e) => {
                l(format!("load filed by {:?}",e));
            },
        };
    }
}
