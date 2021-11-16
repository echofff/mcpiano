mod redpianov2;
use crate::event::Action;
use crate::draw::Area;

pub trait Sheet {
    //fn for_each_track(&self,f: fn(x:usize,usize) -> ());
    fn tr_len(&self) -> usize;
    fn colo(&self, ti: usize) -> usize;
    fn click(&self, x: usize, y: usize, area: Area, act: Action);

    fn save(&self) -> String;
    fn load(&mut self, str: String);

    fn save_comp(&self) -> String;

    fn add_inst(&self, inst: usize, color_s: String);

    fn resize(&self, tar: usize);

    fn shunk(&mut self);
}

pub use redpianov2::RedPianoV2;
