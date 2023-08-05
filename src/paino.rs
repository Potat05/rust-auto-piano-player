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



#[derive(Clone, Copy)]
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

    pub fn char(&self) -> char {
        MIDI_KEY_MAP[self.midi_value as usize]
    }
    pub fn is_valid(&self) -> bool {
        self.char() != '-'
    }
    pub fn is_upper(&self) -> bool {
        // All upper chars are sharp.
        let char = self.char();
        char.is_uppercase() || "!@$%^*(".contains(char)
    }
    pub fn keyboard(&self) -> Keyboard {
        let char = self.char();
        match char {
            '!' => { Keyboard::Number1 }
            '@' => { Keyboard::Number2 }
            '$' => { Keyboard::Number4 }
            '%' => { Keyboard::Number5 }
            '^' => { Keyboard::Number6 }
            '*' => { Keyboard::Number8 }
            '(' => { Keyboard::Number9 }
            _ => { Keyboard::from_str(&char.to_ascii_uppercase().to_string()).unwrap() }
        }
    }

}





pub struct KeyMerger {
    lower: Vec<Key>,
    upper: Vec<Key>,
}

impl KeyMerger {

    pub fn new() -> Self {
        Self { lower: Vec::new(), upper: Vec::new() }
    }

    pub fn sleeps(&self) -> u8 {
        u8::from(!self.lower.is_empty()) + u8::from(!self.upper.is_empty())
    }

    pub fn add_key(&mut self, key: Key) {
        if !key.is_valid() {
            return;
        }
        
        if !key.is_upper() {

            if self.lower.iter().any(|i_key| i_key.midi_value == key.midi_value) {
                return;
            }

            self.lower.push(key);

        } else {

            if self.upper.iter().any(|i_key| i_key.midi_value == key.midi_value) {
                return;
            }

            self.upper.push(key);

        }
    }

    pub fn press_lower_keys(&mut self, time: u64) {

        if self.lower.is_empty() {
            return;
        }


        for key in self.lower.iter() {
            key.keyboard().press();
        }

        thread::sleep(Duration::from_millis(time));

        for key in self.lower.iter() {
            key.keyboard().release();
        }


        self.lower.clear();

    }

    pub fn press_upper_keys(&mut self, time: u64) {

        if self.upper.is_empty() {
            return;
        }


        Keyboard::LeftShift.press();

        for key in self.upper.iter() {
            key.keyboard().press();
        }

        thread::sleep(Duration::from_millis(time));

        for key in self.upper.iter() {
            key.keyboard().release();
        }

        Keyboard::LeftShift.release();


        self.upper.clear();

    }

    pub fn press_keys(&mut self, time: u64) {
        self.press_lower_keys(time);
        self.press_upper_keys(time);
    }

}
