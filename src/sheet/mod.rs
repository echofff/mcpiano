mod redpianov2;
use std::ops::{Deref, DerefMut};

use crate::{
    draw::{Area, Draw},
    event::Event,
};

pub trait Sheet: Deref<Target = CommonData> + DerefMut {
    fn tr_len(&self) -> usize;
    fn click(&mut self, event: Event);

    fn draw(&self, c: &Draw);

    fn save(&self) -> String;
    fn load(&mut self, str: String);

    fn save_comp(&self) -> String;

    fn add_inst(&self, inst: usize, color_s: String);

    fn resize(&mut self, tar: usize) -> usize;
    fn time(&self) -> usize;

    fn key(&mut self, x: usize, y: usize, key: usize) -> bool;
}

pub struct CommonData {
    pub sel_inst: usize,

    // tick per mark
    pub tpm: usize,
}

pub use redpianov2::RedPianoV2;
