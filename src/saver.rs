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
            Ok(c) => {
                self.config = c;
                self.draw_all();
            }
            Err(e) => {
                l(format!("load filed by {:?}", e));
            }
        };
    }
}

impl PianoGlobal {
    pub fn resize(&mut self, n: usize) {
        let max_note = self.tracks.iter().map(|a| a.len()).max().unwrap_throw();
        let tar = max_note.max(n);

        self.tracks
            .iter_mut()
            .for_each(|t| t.resize(tar, Default::default()));

        let width = self.rtd.titlw + self.rtd.cellw * tar as f64;
        let height = self.rtd.cellh * (self.tracks.len() + 25) as f64;

        self.rtd.tablw = width;
        self.rtd.tablh = height;

        self.canv.set_width(width as u32);
        self.canv.set_height(height as u32);

        self.draw_all();
    }
}
