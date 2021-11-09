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

        //self.flesh_insts();
        self.draw_backline();
        self.draw_insts();
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
        let c = &self.cctx;

        self.tracks.iter().enumerate().for_each(|(i, t)| {
            if i != self.rtd.sel_track {
                self.draw_track(t, i);
            }
        });

        self.tracks
            .get(self.rtd.sel_track)
            .map(|t| self.draw_track(t, self.rtd.sel_track));
    }

    pub fn draw_insts(&self) {
        let row_styles = [&"#22443322".into(), &"#44223333".into()];
        let c = &self.cctx;
        let mut yoffset = self.tracks.len() as f64 * self.rtd.cellh + self.rtd.borde;
        (0..25).into_iter().rev().for_each(|i| {
            c.set_fill_style(&"black".into());
            c.fill_text(TITLE[i], 10f64, yoffset + self.rtd.cellh * 0.6f64)
                .unwrap_throw();
            c.set_fill_style(row_styles[i % 2]);
            c.fill_rect(
                self.rtd.borde,
                yoffset,
                self.rtd.tablw - self.rtd.borde * 2f64,
                self.rtd.cellh - self.rtd.borde * 2f64,
            );
            yoffset += self.rtd.cellh;
        })
        //self.insts.iter().enumerate().for_each(|(i, inst)| {
        //    c.set_fill_style(&"black".into());
        //    c.fill_text(TITLE[*inst], 10f64, yoffset + CELLH * 0.6f64)
        //        .unwrap_throw();
        //    c.set_fill_style(row_styles[i % 2]);
        //    c.fill_rect(BORDE, yoffset, TABLW - BORDE * 2f64, CELLH - BORDE * 2f64);
        //    yoffset += CELLH;
        //})
    }

    pub fn draw_backline(&self) {
        let c = &self.cctx;
        let line = [&"#444444FF".into(), &"#00000044".into()];

        let mut x = self.rtd.titlw;
        let mut i = 0;

        while x < self.rtd.tablw {
            c.begin_path();
            c.set_stroke_style(line[if i % 4 == 0 { 0 } else { 1 }]);

            c.move_to(x, 0f64);
            c.line_to(x, self.rtd.tablh);
            x += self.rtd.notew;

            c.stroke();
            i += 1;
        }
    }

    pub fn draw_track(&self, t: &Track, ti: usize) {
        let yoffset = self.rtd.borde + ti as f64 * self.rtd.cellh;
        let row_styles = [&"#22443322".into(), &"#44223333".into()];
        let c = &self.cctx;
        if t.hide {
            c.set_fill_style(&"green".into());
        } else {
            c.set_fill_style(&"blue".into());
        };
        c.fill_rect(
            self.rtd.borde,
            yoffset,
            self.rtd.titlw / 4f64 - self.rtd.borde * 2f64,
            self.rtd.cellh - self.rtd.borde * 2f64,
        );

        c.set_fill_style(&"red".into());
        c.fill_rect(
            self.rtd.borde + self.rtd.titlw / 4f64,
            yoffset,
            self.rtd.titlw / 4f64 - self.rtd.borde * 2f64,
            self.rtd.cellh - self.rtd.borde * 2f64,
        );

        if self.rtd.sel_track == ti {
            c.set_fill_style(&"orange".into());
        } else {
            c.set_fill_style(&"yellow".into());
        }

        c.fill_rect(
            self.rtd.borde + self.rtd.titlw / 2f64,
            yoffset,
            self.rtd.titlw / 4f64 - self.rtd.borde * 2f64,
            self.rtd.cellh - self.rtd.borde * 2f64,
        );

        c.set_fill_style(&t.colo.as_str().into());
        c.fill_rect(
            self.rtd.borde + self.rtd.titlw * 0.75f64,
            yoffset,
            self.rtd.titlw / 4f64 - self.rtd.borde * 2f64,
            self.rtd.cellh - self.rtd.borde * 2f64,
        );

        let mut xoffset = self.rtd.titlw + self.rtd.borde;
        // bg color
        c.set_fill_style(row_styles[ti % 2]);
        c.fill_rect(
            xoffset,
            yoffset,
            self.rtd.tablw - self.rtd.borde * 2f64,
            self.rtd.cellh - self.rtd.borde * 2f64,
        );

        t.notes.iter().enumerate().for_each(|(_, n)| {
            if n.beat == 0 {
                xoffset += self.rtd.cellw;
                return;
            }

            // draw cell
            //c.set_fill_style(ins_styles[2]);
            c.set_fill_style(&"#e53e0f".into());
            c.fill_rect(
                xoffset,
                yoffset + self.rtd.cellh - 3f64,
                self.rtd.cellw - self.rtd.borde * 2f64,
                4f64,
            );

            // draw note
            [0, 1, 2, 3].iter().fold(0b1000, |mask, _| {
                if mask & n.beat != 0 {
                    //c.set_fill_style(col_styles[(j + 1) % 2]);
                    c.set_fill_style(&t.colo.as_str().into());
                    c.fill_rect(
                        xoffset,
                        yoffset,
                        self.rtd.notew - self.rtd.borde * 2f64,
                        self.rtd.notew - self.rtd.borde * 2f64,
                    );
                    if !t.hide {
                        c.fill_rect(
                            xoffset,
                            (self.tracks.len() + n.note as usize) as f64 * self.rtd.cellh,
                            self.rtd.notew - self.rtd.borde * 2f64,
                            self.rtd.notew - self.rtd.borde * 2f64,
                        );
                    }
                }
                xoffset += self.rtd.notew;
                mask >> 1
            });
        });
    }
}
