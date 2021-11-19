use crate::{pianoglobal::Theme, PianoGlobal};

use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::map::*;

pub struct Draw {
    pub cctx: CanvasRenderingContext2d,
    pub canv: HtmlCanvasElement,

    pub cube_w: f64,
    pub cube_h: f64,

    pub titles: usize,

    pub borde: f64,
    pub win_w: f64,
    pub win_h: f64,

    pub wi: usize,
    pub hi: usize,
}

impl Draw {
    pub fn new() -> Draw {
        let (canv, cctx) = Self::scanvas();
        Draw {
            cctx,
            canv,
            cube_w: 20f64,
            cube_h: 20f64,
            borde: 1f64,
            titles: 4,
            win_w: 1900f64,
            win_h: 1000f64,
            hi: 0,
            wi: 0,
        }
    }
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
    pub fn resize(&mut self, l: usize, h: usize) {
        self.wi = l;
        self.hi = h;
        self.win_w = l as f64 * self.cube_w;
        self.win_h = h as f64 * self.cube_h;

        self.canv.set_width(self.win_w as u32);
        self.canv.set_height(self.win_h as u32);
    }
    pub fn clear(&self) {
        self.cctx.clear_rect(0f64, 0f64, self.win_w, self.win_h);
    }
    pub fn rect(&self, x: usize, y: usize, w: usize, h: usize, border: bool) {
        let b = if border { self.borde } else { 0f64 };
        self.cctx.fill_rect(
            x as f64 * self.cube_w + b,
            y as f64 * self.cube_h + b,
            w as f64 * self.cube_w - b * 2f64,
            h as f64 * self.cube_h - b * 2f64,
        );
    }

    pub fn style_fill(&self, col: &str) {
        self.cctx.set_fill_style(&col.into());
    }

    pub fn vert(&self, xi: usize) {
        self.rect(xi, 0, 1, self.hi, false);

        //let ni = ni * 4 + 4;
        //(ni..ni + 4)
        //    .into_iter()
        //    .zip(BEATS.into_iter())
        //    .filter(|(_, b)| b & beat != 0)
        //    .for_each(|(ni, _)| {
        //        self.rect(ni, 0, 1, len, false);
        //    });
    }

    pub fn down_line(&self, x: usize, y: usize) {
        self.cctx.fill_rect(
            (x * 4 + 4) as f64 * self.cube_w + self.borde,
            y as f64 * self.cube_h + 0.75f64 * self.cube_h,
            self.cube_w * 4f64 - self.borde * 2f64,
            self.cube_h * 0.25,
        );
    }
    pub fn backline(&self, len: usize) {
        let c = &self.cctx;
        let line = [&"#666666".into(), &"#cccccc".into()];

        c.begin_path();

        // draw lighter line
        c.set_stroke_style(line[1]);
        (0..len).into_iter().for_each(|i| {
            let x = (i + 4) as f64 * self.cube_w;
            c.move_to(x, 0f64);
            c.line_to(x, self.win_h);
        });

        // draw darker line
        c.set_stroke_style(line[0]);
        (0..len / 4).into_iter().for_each(|i| {
            let x = (i * 4 + 4) as f64 * self.cube_w;
            c.move_to(x, 0f64);
            c.line_to(x, self.win_h);
        });

        c.stroke();
    }

    pub fn draw_insts(&self, theme: &Theme, trs: usize, time: usize) {
        let c = &self.cctx;

        // draw control-pane and secquene-paneb background
        (0..trs).into_iter().for_each(|ti| {
            self.style_fill(&theme.track_row[ti % theme.track_row.len()]);
            //c.fill_rect(0f64, rt.cellh * ti as f64, rt.tablw, rt.cellh);
            self.rect(0, ti, 4 + time, 1, false);
        });

        // draw edit-pane background
        (0..25).into_iter().for_each(|i| {
            self.style_fill(&theme.note_row[(24 - i) % theme.note_row.len()]);
            //c.fill_rect(0f64, y, rt.tablw, rt.cellh);
            self.rect(0, trs + i, 4 + time, 1, false);

            self.style_fill("black");
            let y = (trs + i) as f64 * self.cube_h;

            self.cctx
                .fill_text(TITLE[24 - i], 10f64, y + self.cube_h * 0.6f64)
                .unwrap_throw();
        })
    }

    fn cube_inplace(&self, x: usize, y: usize, area: Area) {}
}

#[wasm_bindgen]
impl PianoGlobal {
    pub fn draw_all(&mut self) {
        self.cctx.clear();

        self.cctx
            .draw_insts(&self.theme, self.sheet.tr_len(), self.sheet.time());

        self.cctx.backline(self.sheet.time());

        self.draw_hightlight();

        self.draw_tracks();

        self.draw_hover();
    }
}

impl PianoGlobal {
    fn draw_hover(&self) {
        let (x, y) = self.pos;
        self.cctx.style_fill(&self.theme.hover);
        self.cctx.rect(x & !0b11, y, 4, 1, false);
        self.cctx.rect(x, y, 1, 1, false);
    }
    fn draw_tracks(&self) {
        self.sheet.draw(&self.cctx);
    }

    fn draw_hightlight(&self) {
        let len = self.sheet.time();
        //self.cctx.style_fill(&self.theme.sel);
        //self.select_hl.iter().for_each(|(ni, beat)| {
        //    self.cctx.vert(*ni, *beat, len);
        //});
        self.cctx.style_fill(&self.theme.error);
        self.sheet.error.iter().for_each(|x| {
            self.cctx.vert(*x);
        });

        self.cctx.style_fill(&self.theme.play);
        let (ni, beat) = (self.play_bt >> 2, 0b1000 >> (self.play_bt & 0b11));
        //self.cctx.vert(ni, beat, len);

        self.cctx.style_fill(&self.theme.hover);
        //let (ni, beat) = ((self.pos.0 >> 2) - 1, 0b1000 >> (self.pos.0 & 0b11));
        self.cctx.vert(self.pos.0);
    }
}

pub enum Area {
    TrackControl,
    TrackSecquence,
    EditPlane,
    InstTitle,
}
