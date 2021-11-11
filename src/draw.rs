use crate::track::Track;
use crate::PianoGlobal;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;

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
    pub fn scanvas() -> (HtmlCanvasElement, CanvasRenderingContext2d) {
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

    pub fn draw_tracks(&self) {
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

    pub fn draw_insts(&self) {
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

    pub fn draw_backline(&self) {
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

    pub fn draw_track(&self, t: &Track, ti: usize) {
        let theme = &self.theme;

        // draw control part
        let area = Area::TrackControl;
        let is_selected = self.rtd.sel_inst == t.inst;

        self.draw_cube(&theme.control[t.hide as usize], &area, 0, ti);
        self.draw_cube(&theme.control[2], &area, 1, ti);
        self.draw_cube(&theme.control[3 + is_selected as usize], &area, 2, ti);
        self.draw_cube(&t.colo, &area, 3, ti);

        t.notes.iter().enumerate().for_each(|(ni, n)| {
            let area = Area::TrackSecquence;
            if n.beat == 0 {
                return;
            }

            // draw under line for every note that is not empty.
            self.draw_down_line(ni, ti);

            (0..4).into_iter().for_each(|bi| {
                if 0b1000 >> bi & n.beat != 0 {
                    self.draw_cube(&t.colo, &area, ni * 4 + bi as usize, ti);
                    if !t.hide {
                        self.draw_cube(
                            &t.colo,
                            &Area::EditPlane,
                            ni * 4 + bi,
                            24 - n.note as usize,
                        );
                    }
                }
            });
        });
    }

    pub fn draw_cube(&self, color: &JsValue, area: &Area, x: usize, y: usize) {
        // use different offset for different areas.
        let (xoffset, yoffset) = match area {
            Area::TrackControl => (0f64, 0f64),
            Area::TrackSecquence => (self.rtd.titlw, 0f64),
            Area::EditPlane => (self.rtd.titlw, self.tracks.len() as f64 * self.rtd.cellh),
            Area::InstTitle => (0f64, self.tracks.len() as f64 * self.rtd.cellh),
        };
        let cv = &self.rtd;

        self.cctx.set_fill_style(color);
        self.cctx.fill_rect(
            x as f64 * cv.notew + xoffset + cv.borde,
            y as f64 * cv.cellh + yoffset + cv.borde,
            self.rtd.notew - cv.borde * 2f64,
            self.rtd.cellh - cv.borde * 4f64,
        );
    }

    pub fn draw_down_line(&self, x: usize, y: usize) {
        let cv = &self.rtd;
        self.cctx.set_fill_style(&self.theme.control[5]);
        self.cctx.fill_rect(
            x as f64 * cv.cellw + cv.titlw + cv.borde,
            y as f64 * cv.cellh + 0.75f64 * cv.cellh,
            cv.cellw - cv.borde * 2f64,
            cv.cellh * 0.25,
        );
    }

    pub fn draw_hightlight(&self) {
        self.rtd.select_hl.iter().for_each(|(ni, beat)| {
            self.draw_vert(&self.theme.sel, *ni, *beat);
        });
        self.rtd.error_hl.iter().for_each(|(ni, beat)| {
            self.draw_vert(&self.theme.error, *ni, *beat);
        });

        let (ni, beat) = (self.rtd.play_bt >> 2, 0b1000 >> (self.rtd.play_bt & 0b11));
        self.draw_vert(&self.theme.play, ni, beat)
    }

    pub fn draw_vert(&self, color: &JsValue, ni: usize, beat: u8) {
        self.cctx.set_fill_style(color);
        (0..4).into_iter().for_each(|i| {
            if beat & (0b1000 >> i) != 0 {
                self.cctx.fill_rect(
                    self.rtd.titlw + ni as f64 * self.rtd.cellw + i as f64 * self.rtd.notew,
                    0f64,
                    self.rtd.notew,
                    self.rtd.tablh,
                )
            }
        })
    }
}

pub enum Area {
    TrackControl,
    TrackSecquence,
    EditPlane,
    InstTitle,
}
