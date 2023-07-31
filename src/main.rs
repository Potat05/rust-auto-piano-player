mod paino;
mod song;
mod r#macro;
mod sheet;

use std::{process::ExitCode, time::Duration, fs};
use mki::Keyboard;

use crate::{song::Song, sheet::sheet_to_macro, r#macro::Macro};



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

    if song.name.is_some() {
        println!("Loaded \"{}\".", song.name.unwrap());
    } else {
        println!("Loaded.");
    }
    if song.description.is_some() {
        println!("{}", song.description.unwrap());
    }
    


    println!();
    println!("NUMPAD0 = Exit program.");
    println!("NUMPAD1 = Start playing song.");
    println!("NUMPAD2 = Stop playing song.");
    println!();



    let mut r#macro: Macro = sheet_to_macro(song.sheet);



    'mainloop: loop {
        if Keyboard::is_pressed(&Keyboard::Numpad0) {
            println!("Exiting.");
            break 'mainloop;
        }

        if Keyboard::is_pressed(&Keyboard::Numpad1) {

            println!("Started.");

            while !r#macro.finished() {

                if Keyboard::is_pressed(&Keyboard::Numpad0) {
                    println!("Exiting.");
                    break 'mainloop;
                }

                if Keyboard::is_pressed(&Keyboard::Numpad2) {
                    println!("Stopping.");
                    continue 'mainloop;
                }

                while !r#macro.tick_finished() {
                    r#macro.tick();
                }

            }

            println!("Finished.");

        }

        std::thread::sleep(Duration::from_millis(100));
    }



    return ExitCode::SUCCESS;
}
