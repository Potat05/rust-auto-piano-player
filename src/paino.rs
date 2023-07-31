use std::{thread, time::Duration, str::FromStr};

use mki::Keyboard;



/**
 * \\- for none.  
 * Every virtual piano I found has this map. So it's probably fine.  
 */
static MIDI_KEY_MAP: [char; 128] = [
    // Horizontal is note.
    // Vertical is octave.
//   C    C#   D    D#   E    F    F#   G    G#   A    A#   B 
    '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', // -1
    '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', // 0
    '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', // 1
    '1', '!', '2', '@', '3', '4', '$', '5', '%', '6', '^', '7', // 2
    '8', '*', '9', '(', '0', 'q', 'Q', 'w', 'W', 'e', 'E', 'r', // 3
    't', 'T', 'y', 'Y', 'u', 'i', 'I', 'o', 'O', 'p', 'P', 'a', // 4
    's', 'S', 'd', 'D', 'f', 'g', 'G', 'h', 'H', 'j', 'J', 'k', // 5
    'l', 'L', 'z', 'Z', 'x', 'c', 'C', 'v', 'V', 'b', 'B', 'n', // 6
    'm', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', // 7
    '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', // 8
    '-', '-', '-', '-', '-', '-', '-', '-'                      // 9
];



pub struct Key {
    pub midi_value: u8,
}



impl Key {
    pub fn new(midi_value: u8) -> Self {
        return Self { midi_value };
    }
    pub fn new_from_key(midi_key: char) -> Self {
        for (i, key) in MIDI_KEY_MAP.iter().enumerate() {
            if *key == midi_key {
                return Self::new(i.try_into().unwrap());
            }
        }
        return Self::new(0);
    }

    pub fn press(&mut self) {
        let key_char = MIDI_KEY_MAP[self.midi_value as usize];

        if key_char == '-' {
            return;
        }

        let is_special = "!@$%^*(".contains(key_char);

        let key = match key_char {
            '!' => { Keyboard::Number1 }
            '@' => { Keyboard::Number2 }
            '$' => { Keyboard::Number4 }
            '%' => { Keyboard::Number5 }
            '^' => { Keyboard::Number6 }
            '*' => { Keyboard::Number8 }
            '(' => { Keyboard::Number9 }
            _ => { Keyboard::from_str(&key_char.to_ascii_uppercase().to_string()).unwrap() }
        };
        let upper = key_char.is_uppercase() || is_special;

        if !upper {

            key.press();
            thread::sleep(Duration::from_millis(15));
            key.release();

        } else {

            Keyboard::LeftShift.press();
            key.press();
            thread::sleep(Duration::from_millis(15));
            Keyboard::LeftShift.release();
            key.release();

        }

    }
}


