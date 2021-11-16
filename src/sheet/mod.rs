mod redpianov2;

pub trait Sheet {
    //fn for_each_track(&self,f: fn(x:usize,usize) -> ());
    fn tr_len(&self) -> usize;
    fn colo(&self,usize) -> usize;
}

pub use redpianov2::RedPianoV2;
