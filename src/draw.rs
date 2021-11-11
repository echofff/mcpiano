use crate::track::Track;
use crate::PianoGlobal;

use wasm_bindgen::prelude::*;

use crate::map::*;

#[wasm_bindgen]
impl PianoGlobal {
    pub fn draw_all(&mut self) {
        self.cctx
            .clear_rect(0f64, 0f64, self.rtd.tablw, self.rtd.tablh);

        self.draw_insts();
        self.draw_backline();
        self.draw_hightlight();
        self.draw_tracks();
    }
}

impl PianoGlobal {
    fn draw_tracks(&self) {
        self.tracks
            .iter()
            .enumerate()
            .filter(|(_, t)| t.inst != self.rtd.sel_inst)
            .for_each(|(i, t)| self.draw_track(t, i));
        self.tracks
            .iter()
            .enumerate()
            .filter(|(_, t)| t.inst == self.rtd.sel_inst)
            .for_each(|(i, t)| self.draw_track(t, i));
    }

    fn draw_insts(&self) {
        let c = &self.cctx;
        let theme = &self.theme;
        let rt = &self.rtd;

        // draw control-pane and secquene-paneb background
        (0..self.tracks.len()).into_iter().for_each(|ti| {
            c.set_fill_style(&theme.track_row[ti % theme.track_row.len()]);
            c.fill_rect(0f64, rt.cellh * ti as f64, rt.tablw, rt.cellh);
        });

        // draw edit-pane background
        (0..25).into_iter().for_each(|i| {
            let y = (self.tracks.len() + i) as f64 * rt.cellh;

            c.set_fill_style(&theme.note_row[(24 - i) % theme.note_row.len()]);
            c.fill_rect(0f64, y, rt.tablw, rt.cellh);

            c.set_fill_style(&"black".into());
            c.fill_text(TITLE[24 - i], 10f64, y + rt.cellh * 0.6f64)
                .unwrap_throw();
        })
    }

    fn draw_backline(&self) {
        let c = &self.cctx;
        let line = [&"#666666".into(), &"#cccccc".into()];

        c.begin_path();

        // draw lighter line
        c.set_stroke_style(line[1]);
        (0..self.rtd.maxnote * 4).into_iter().for_each(|i| {
            let x = self.rtd.titlw + i as f64 * self.rtd.notew;
            c.move_to(x, 0f64);
            c.line_to(x, self.rtd.tablh);
        });

        // draw darker line
        c.set_stroke_style(line[0]);
        (0..self.rtd.maxnote).into_iter().for_each(|i| {
            let x = self.rtd.titlw + i as f64 * self.rtd.cellw;
            c.move_to(x, 0f64);
            c.line_to(x, self.rtd.tablh);
        });

        c.stroke();
    }

    fn draw_track(&self, t: &Track, ti: usize) {
        let theme = &self.theme;

        // draw control part
        let area = Area::TrackControl;
        let is_selected = self.rtd.sel_inst == t.inst;

        self.cctx.set_fill_style(&theme.control[t.hide as usize]);
        self.draw_beat(&area, 0, ti, 0b1000);

        self.cctx.set_fill_style(&theme.control[2]);
        self.draw_beat(&area, 0, ti, 0b0100);

        self.cctx
            .set_fill_style(&theme.control[3 + is_selected as usize]);
        self.draw_beat(&area, 0, ti, 0b0010);

        self.cctx.set_fill_style(&t.colo);
        self.draw_beat(&area, 0, ti, 0b0001);

        t.notes
            .iter()
            .enumerate()
            .filter(|(_, n)| n.beat != 0)
            .for_each(|(ni, n)| {
                // draw under line for every note that is not empty.
                self.draw_down_line(ni, ti);
                self.draw_beat(&Area::TrackSecquence, ni, ti, n.beat);

                if !t.hide {
                    self.draw_beat(&Area::EditPlane, ni, 24 - n.note as usize, n.beat);
                }
            });
    }

    fn draw_beat(&self, area: &Area, x: usize, y: usize, beat: u8) {
        // use different offset for different areas.
        let (x, y) = match area {
            Area::TrackControl => (x, y),
            Area::TrackSecquence => (x * 4 + 4, y),
            Area::EditPlane => (x * 4 + 4, y + self.tracks.len()),
            Area::InstTitle => (x * 4, y + self.tracks.len()),
        };
        let cv = &self.rtd;

        let y = y as f64 * cv.cellh + cv.borde;
        let w = cv.notew - cv.borde * 2f64;
        let h = cv.cellh - cv.borde * 4f64;

        (x..x + 4)
            .into_iter()
            .zip(BEATS.into_iter())
            .filter(|(_, b)| b & beat != 0)
            .for_each(|(x, _)| {
                let x = x as f64 * cv.notew + cv.borde;
                self.cctx.fill_rect(x, y, w, h);
            })
    }

    fn draw_down_line(&self, x: usize, y: usize) {
        let cv = &self.rtd;
        self.cctx.fill_rect(
            x as f64 * cv.cellw + cv.titlw + cv.borde,
            y as f64 * cv.cellh + 0.75f64 * cv.cellh,
            cv.cellw - cv.borde * 2f64,
            cv.cellh * 0.25,
        );
    }

    fn draw_hightlight(&self) {
        self.cctx.set_fill_style(&self.theme.sel);
        self.rtd.select_hl.iter().for_each(|(ni, beat)| {
            self.draw_vert(*ni, *beat);
        });
        self.cctx.set_fill_style(&self.theme.error);
        self.rtd.error_hl.iter().for_each(|(ni, beat)| {
            self.draw_vert(*ni, *beat);
        });

        self.cctx.set_fill_style(&self.theme.play);
        let (ni, beat) = (self.rtd.play_bt >> 2, 0b1000 >> (self.rtd.play_bt & 0b11));
        self.draw_vert(ni, beat)
    }

    fn draw_vert(&self, ni: usize, beat: u8) {
        let ni = ni * 4 + 4;
        (ni..ni + 4)
            .into_iter()
            .zip(BEATS.into_iter())
            .filter(|(_, b)| b & beat != 0)
            .for_each(|(ni, _)| {
                self.cctx.fill_rect(
                    ni as f64 * self.rtd.notew,
                    0f64,
                    self.rtd.notew,
                    self.rtd.tablh,
                )
            });
    }
}

pub enum Area {
    TrackControl,
    TrackSecquence,
    EditPlane,
    InstTitle,
}
