
use serde_json::Number;

use crate::{song::{SheetSongData, SheetSongDataDelay}, r#macro::Macro, paino::Key};



pub fn sheet_to_macro(sheet: SheetSongData) -> Macro {

    let default_delay = SheetSongDataDelay {  key: None, space: None, pause: None, fast: None };

    let delay = sheet.delay.unwrap_or(default_delay);
    let delay_key = delay.key.unwrap_or(Number::from(200)).as_i64().unwrap();
    let delay_space = delay.space.unwrap_or(Number::from(200)).as_i64().unwrap();
    let delay_pause = delay.pause.unwrap_or(Number::from(400)).as_i64().unwrap();
    let delay_fast = delay.fast.unwrap_or(Number::from(50)).as_i64().unwrap();

    let string = sheet.string;



    let mut r#macro: Macro = Macro::new();



    let mut is_in_bracket = false;

    for char in string.chars().into_iter() {
        match char {
            '|' => {
                r#macro.add_delay(delay_pause);
            }
            '[' => {
                is_in_bracket = true;
            }
            ']' => {
                is_in_bracket = false;
                r#macro.add_delay(delay_key);
            }
            ' ' => {
                if is_in_bracket {
                    r#macro.add_delay(delay_fast);
                } else if delay_space > 0 {
                    r#macro.add_delay(delay_space);
                }
            }
            _ => {

                r#macro.add_key(Key::new_from_key(char));

                if !is_in_bracket {
                    r#macro.add_delay(delay_key);
                }
                
            }
        }
    }



    r#macro

}


