use serde::Serialize;
use serde_json::{json, Value};

use crate::map::EMPTY;

#[derive(Serialize, Debug)]
pub struct Item {
    pub id: &'static str,
    pub tag: Value,
    pub Slot: usize,
    pub Count: usize,
}

#[derive(Serialize)]
pub struct Shulk {
    pub id: &'static str,
    pub tag: Value,
    pub Slot: usize,
    pub Count: usize,
}

#[derive(Serialize)]
pub struct Chest {
    pub display: Value,
    pub BlockEntityTag: Value,
}

pub fn items2String(items: &mut Vec<Item>, name: String) -> String {
    while items.len() % 27 != 0 {
        items.push(Item {
            id: EMPTY[items.len() % 2],
            tag: json!({"display":{ "Name": "{\"text\": \"2_G#   1\"}" }}),
            Slot: 0,
            Count: 1,
        })
    }

    items
        .iter_mut()
        .enumerate()
        .for_each(|(i, it)| it.Slot = i % 27);

    let mut shulks = Vec::new();
    for i in 0..items.len() / 27 {
        shulks.push(Shulk {
            id: "minecraft:shulker_box",
            tag: json!( { "BlockEntityTag": { "Items": items[i*27..i*27+27] } }),
            Slot: i,
            Count: 1,
        })
    }

    let chest = Chest {
        display: json!({ "Name": format!("{{\"text\":\"{}\"}}", name) }),
        BlockEntityTag: json!({ "Items": shulks }),
    };

    //String::new()
    match serde_json::to_string(&chest) {
        Ok(e) => e,
        Err(e) => {
            format!("--{:?}--", e)
        }
    }
}
