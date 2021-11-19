use wasm_bindgen::{prelude::*, throw_str, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{AudioBuffer, AudioContext, Request, RequestInit, Response};

#[derive(serde::Deserialize)]
pub struct NoteBox {
    pub name: String,
    pub hash: String,
    //size: usize,
    #[serde(skip_deserializing)]
    pub audio: Option<AudioBuffer>,
}

pub struct Player {
    pub volumn: f32,
    pub actx: AudioContext,
    pub sounds: Vec<NoteBox>,
    pub pause: bool,
}

impl Player {
    pub async fn new() -> Player {
        let actx = AudioContext::new().unwrap_throw();

        //let url = "conf.json";

        let request = Request::new_with_str_and_init("conf.json", RequestInit::new().method("GET"))
            .expect_throw("req");

        let window = web_sys::window().unwrap();

        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .expect_throw("resp_value");

        // `resp_value` is a `Response` object.
        assert!(resp_value.is_instance_of::<Response>());

        let resp: Response = resp_value.dyn_into().unwrap();

        // Convert this other `Promise` into a rust `Future`.
        let json = JsFuture::from(resp.json().expect_throw("run->json2")).await;
        //.expect_throw("run->json");
        let json = if let Err(e) = json {
            throw_str(format!("{:?}", e).as_str());
        } else {
            json.unwrap()
        };

        // Use serde to parse the JSON into a struct.
        let mut sounds: Vec<NoteBox> = json.into_serde().unwrap();

        for ele in &mut sounds {
            //ele.genab(&ctx).await.expect_throw("run-> for");
            let a = ele.genab(&actx).await;
            if let Err(a) = a {
                throw_str(format!("---------{:?}", a).as_str());
            }
        }

        Player {
            pause: true,
            volumn: 0.7f32,
            actx,
            sounds,
        }
    }

    pub fn set_volumn(&mut self, volumn: f32) {
        self.volumn = volumn;
    }
    pub fn play(&self, inst: usize, note: usize) {
        let a = self.actx.create_buffer_source().unwrap_throw();
        let g = self.actx.create_gain().unwrap_throw();

        a.set_buffer(self.sounds[inst as usize].audio.as_ref());
        a.detune().set_value((note as f32 - 12f32) * 100f32);
        g.gain().set_value(self.volumn);

        a.connect_with_audio_node(&g).expect_throw("connect play");
        g.connect_with_audio_node(&self.actx.destination())
            .expect_throw("connect play");

        a.start().expect_throw("start play");
    }

    pub fn play_start(&mut self) -> bool {
        self.pause ^= true;
        !self.pause
    }

    //pub fn play_continue(&self) -> bool {
    //    //self.sheet.time() / 4 > self.play_bt >> 2
    //}

    pub fn play_stage(&mut self) -> bool {
        false
        //        let (ni, beat) = (self.play_bt >> 2, 0b1000 >> (self.play_bt & 0b11));
        //
        //        if self.maxnote > ni {
        //            self.play_bt += 1;
        //            self.draw_all();
        //            self.tracks
        //                .iter()
        //                .filter_map(|t| t.get(ni).map(|n| (t.inst, n)))
        //                .filter(|(_, n)| n.beat & beat != 0)
        //                .for_each(|(inst, n)| {
        //                    self.play(inst as u8, n.note);
        //                });
        //            true
        //        } else {
        //            self.pause = true;
        //            self.play_bt = 0;
        //            self.draw_all();
        //            false
        //        }
    }
}

impl NoteBox {
    pub async fn genab(&mut self, ctx: &AudioContext) -> Result<(), JsValue> {
        let mut opts = RequestInit::new();
        opts.method("GET");
        let urls = format!("./res/{}", self.hash);

        let req = Request::new_with_str_and_init(&urls, &opts)?;
        let window = web_sys::window().unwrap();

        let resp: Response = JsFuture::from(window.fetch_with_request(&req))
            .await?
            .dyn_into()?;
        let ab = JsFuture::from(resp.array_buffer()?).await?.dyn_into()?;

        let ab = JsFuture::from(ctx.decode_audio_data(&ab)?)
            .await?
            .dyn_into()?;

        self.audio = Some(ab);

        Ok(())
    }
}
pub const KEYM: [usize; 25] = [
    192, 9, 49, 81, 50, 87, 69, 52, 82, 53, 84, 89, 55, 85, 56, 73, 57, 79, 80, 173, 219, 61, 221,
    220, 8,
];
