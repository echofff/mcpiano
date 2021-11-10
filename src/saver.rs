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

    pub fn save_comp(&self) -> String {
        let length = self.tracks.len() * (self.tracks[0].len() + 20);
        let mut res = String::with_capacity(length);

        self.tracks.iter().for_each(|t| {
            res.insert_str(res.len(), "Z00");
            res.insert_str(res.len(), t.colo_s.as_str());
            res.push(INST[t.inst as usize] as char);
            t.iter().for_each(|n| {
                res.push(NOTE[n.note as usize] as char);
                res.push(BEAT[n.beat as usize] as char);
            });
            res.push('\r');
            res.push('\n');
        });

        res
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

    pub fn add_track(&mut self, inst: i32, colo_s: String) {
        if inst < 0 {
            return;
        }
        self.tracks.push(Track {
            inst: inst as usize,
            hide: false,
            colo: JsValue::from(colo_s.as_str()),
            colo_s,
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
const NOTE: &[u8; 25] = b"ABCDEFGHIJKLMNOPQRSTUVWXY";
const BEAT: &[u8; 16] = b"0123456789abcdef";
const INST: &[u8; 18] = b"ghijklmnopqrstuvwx";
