mod paino;
mod song;

use std::{process::ExitCode, time::Duration, fs};
use mki::Keyboard;
use paino::Key;
use serde_json::{self, Number};

use crate::song::Song;



fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 || args[1].len() == 0 {
        println!("No file was inputted.");
        return ExitCode::FAILURE;
    }

    let file_path = &args[1];

    if fs::metadata(file_path).is_err() {
        println!("Error while reading file.");
        return ExitCode::FAILURE;
    }

    let song_string = fs::read_to_string(file_path).unwrap();
    let song: Song = serde_json::from_str(&song_string).unwrap();

    println!("Loaded \"{}\"", song.name);
    if song.description.is_some() {
        println!("{}", song.description.unwrap());
    }
    
    let sheet = song.sheet;
    
    let delay = sheet.delay;
    let delay_key = delay.key.unwrap_or(Number::from(200)).as_u64().unwrap();
    let delay_space = delay.space.unwrap_or(Number::from(200)).as_u64().unwrap();
    let delay_pause = delay.pause.unwrap_or(Number::from(400)).as_u64().unwrap();
    let delay_fast = delay.fast.unwrap_or(Number::from(50)).as_u64().unwrap();

    let string = sheet.string;



    println!();
    println!("NUMPAD0 = Exit program.");
    println!("NUMPAD1 = Start playing song.");
    println!("NUMPAD2 = Stop playing song.");
    println!();



    'mainloop: loop {
        if Keyboard::is_pressed(&Keyboard::Numpad0) {
            println!("Exiting.");
            break 'mainloop;
        }

        if Keyboard::is_pressed(&Keyboard::Numpad1) {

            println!("Started.");

            let mut is_in_bracket = false;
            let mut num_key_bracket = 0;

            for char in string.chars().into_iter() {

                if Keyboard::is_pressed(&Keyboard::Numpad0) {
                    println!("Exiting.");
                    break 'mainloop;
                }

                if Keyboard::is_pressed(&Keyboard::Numpad2) {
                    println!("Stopping.");
                    continue 'mainloop;
                }

                match char {
                    '|' => {
                        std::thread::sleep(Duration::from_millis(delay_pause));
                    }
                    '[' => {
                        is_in_bracket = true;
                        num_key_bracket = 0;
                    }
                    ']' => {
                        is_in_bracket = false;
                        // TODO - It is assumed that a key is played inside this, don't assume anything.
                        std::thread::sleep(Duration::from_millis(delay_key - 15 * num_key_bracket));
                    }
                    ' ' => {
                        if is_in_bracket {
                            num_key_bracket = 0;
                            std::thread::sleep(Duration::from_millis(delay_fast));
                        } else if delay_space > 0 {
                            std::thread::sleep(Duration::from_millis(delay_space));
                        }
                    }
                    _ => {
                        Key::new_from_key(char).press();
                        if !is_in_bracket {
                            std::thread::sleep(Duration::from_millis(delay_key - 15));
                        } else {
                            num_key_bracket += 1;
                        }
                    }
                }

            }

            println!("Finished.");

        }

        std::thread::sleep(Duration::from_millis(100));
    }



    return ExitCode::SUCCESS;
}
