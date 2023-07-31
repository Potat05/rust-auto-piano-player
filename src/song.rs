
use serde::{Deserialize, Serialize};
use serde_json::Number;



#[derive(Serialize, Deserialize)]
pub struct SheetSongDataDelay {
    pub key: Option<Number>,
    pub space: Option<Number>,
    pub pause: Option<Number>,
    pub fast: Option<Number>,
}

#[derive(Serialize, Deserialize)]
pub struct SheetSongData {
    pub delay: Option<SheetSongDataDelay>,
    pub string: String,
}



#[derive(Serialize, Deserialize)]
pub struct MacroSongEvent {
    pub r#type: Number,
    pub value: Number,
}


#[derive(Serialize, Deserialize)]
pub struct MacroSongData {
    pub events: Vec<MacroSongEvent>,
}



#[derive(Serialize, Deserialize)]
pub struct Song {
    pub name: Option<String>,
    pub description: Option<String>,
    pub sheet: Option<SheetSongData>,
    pub r#macro: Option<MacroSongData>,
}


