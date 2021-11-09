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

        //context.fill_rect(20f64, 20f64, 50f64, 50f64);
        //let a = JsString::from("green");
        //let a = JsString::from("#88888820");
        //context.set_fill_style(&a.into());
        //context.fill_rect(50f64, 50f64, 330f64, 330f64);
    }

    //pub fn flesh_insts(&mut self) {
    //    self.tracks.iter_mut().for_each(|t| {
    //        t.notes.iter_mut().for_each(|n| {
    //            if let Some(i) = self.insts.get(n.insi as usize) {
    //                if *i == (n.inst, n.note) {
    //                    return;
    //                }
    //            }

    //            if let Some((insi, _)) = self
    //                .insts
    //                .iter()
    //                .enumerate()
    //                .find(|e| *e.1 == (n.inst, n.note))
    //            {
    //                //log("find");
    //                n.borrow_mut().insi = insi as u8;
    //            }

    //            //n.borrow_mut().
    //        })
    //    })
    //}

    pub fn draw_tracks(&self) {
        let c = &self.cctx;

        let row_styles = [&"#22443322".into(), &"#44223333".into()];
        //let col_styles = [&"#22448322".into(), &"#22883333".into()];
        let ins_styles = [
            &"#9a9dea".into(),
            &"#7bd4ca".into(),
            &"#e53e0f".into(),
            &"#18d579".into(),
            &"#8cea44".into(),
            &"#645122".into(),
        ];

        let mut yoffset = 0f64 + self.rtd.borde;
        self.tracks.iter().enumerate().for_each(|(i, t)| {
            let mut xoffset = self.rtd.titlw + self.rtd.borde;
            // bg color
            c.set_fill_style(row_styles[i % 2]);
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
                c.set_fill_style(ins_styles[2]);
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
                        c.set_fill_style(ins_styles[i]);
                        c.fill_rect(
                            xoffset,
                            yoffset,
                            self.rtd.notew - self.rtd.borde * 2f64,
                            self.rtd.notew - self.rtd.borde * 2f64,
                        );
                        c.fill_rect(
                            xoffset,
                            (self.tracks.len() + n.note as usize) as f64 * self.rtd.cellh,
                            self.rtd.notew - self.rtd.borde * 2f64,
                            self.rtd.notew - self.rtd.borde * 2f64,
                        );
                    }
                    xoffset += self.rtd.notew;
                    mask >> 1
                });
            });
            yoffset += self.rtd.cellh;
        })
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
}
