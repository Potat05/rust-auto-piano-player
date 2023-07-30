
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
    pub delay: SheetSongDataDelay,
    pub string: String,
}



#[derive(Serialize, Deserialize)]
pub struct Song {
    pub name: String,
    pub description: Option<String>,
    // TODO - Macro data type.
    pub sheet: SheetSongData,
}


