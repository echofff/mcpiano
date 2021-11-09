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

    pub fn add_track(&mut self, inst: i32, colo: String) {
        if inst < 0 {
            return;
        }
        self.tracks.push(Track {
            inst: inst as usize,
            hide: false,
            colo,
            notes: Default::default(),
        });
        self.resize(0);
    }

    pub fn resize(&mut self, n: i32) {
        if n < 0 {
            return;
        }
        self.shunk();
        let max_note = self.tracks.iter().map(|a| a.len()).max().unwrap_throw();
        let tar = max_note.max(n as usize);

        self.tracks
            .iter_mut()
            .for_each(|t| t.resize(tar, Default::default()));

        self.rtd.maxnote = tar * 4;

        let width = self.rtd.titlw + self.rtd.cellw * tar as f64;
        let height = self.rtd.cellh * (self.tracks.len() + 25) as f64;

        self.rtd.tablw = width;
        self.rtd.tablh = height;

        self.canv.set_width(width as u32);
        self.canv.set_height(height as u32);

        self.draw_all();
        //alert("asdfasdf");
    }

    pub fn shunk(&mut self) {
        let mut last = self.rtd.maxnote / 4 - 1;

        while self.tracks.iter().all(|t| {
            if let Some(Note { beat: 0, .. }) = t.get(last) {
                true
            } else {
                false
            }
        }) {
            self.tracks.iter_mut().for_each(|t| {
                t.pop();
            });
            last -= 1;
        }
    }
}
