use std::{thread, time::Duration};

use mki::{Sequence, Keyboard};



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
    midi_value: u8
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

        // TODO: Don't do it this way.
        // These is incredibly stupid and dumb
        // !@$%^*( seems to be broken?
        match key_char {
            '!' => {
                Keyboard::LeftShift.press();
                Keyboard::Number1.press();
                thread::sleep(Duration::from_millis(15));
                Keyboard::LeftShift.release();
                Keyboard::Number1.release();
            }
            '@' => {
                Keyboard::LeftShift.press();
                Keyboard::Number2.press();
                thread::sleep(Duration::from_millis(15));
                Keyboard::LeftShift.release();
                Keyboard::Number2.release();
            }
            '$' => {
                Keyboard::LeftShift.press();
                Keyboard::Number4.press();
                thread::sleep(Duration::from_millis(15));
                Keyboard::LeftShift.release();
                Keyboard::Number4.release();
            }
            '%' => {
                Keyboard::LeftShift.press();
                Keyboard::Number5.press();
                thread::sleep(Duration::from_millis(15));
                Keyboard::LeftShift.release();
                Keyboard::Number5.release();
            }
            '^' => {
                Keyboard::LeftShift.press();
                Keyboard::Number6.press();
                thread::sleep(Duration::from_millis(15));
                Keyboard::LeftShift.release();
                Keyboard::Number6.release();
            }
            '*' => {
                Keyboard::LeftShift.press();
                Keyboard::Number8.press();
                thread::sleep(Duration::from_millis(15));
                Keyboard::LeftShift.release();
                Keyboard::Number8.release();
            }
            '(' => {
                Keyboard::LeftShift.press();
                Keyboard::Number9.press();
                thread::sleep(Duration::from_millis(15));
                Keyboard::LeftShift.release();
                Keyboard::Number9.release();
            }
            _ => {
                Sequence::text(&key_char.to_string()).unwrap().send();
                thread::sleep(Duration::from_millis(15));
            }
        }

    }
}


