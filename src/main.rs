mod paino;
mod song;
mod r#macro;
mod sheet;

use std::{process::ExitCode, time::Duration, fs};
use indicatif::{ProgressBar, ProgressStyle};
use mki::Keyboard;

use crate::{song::Song, sheet::sheet_to_macro, r#macro::Macro};



fn format_duration(duration: u64) -> String {
    if duration >= 3_600_000 {
        format!(
            "{:0>2}:{:0>2}:{:0>2}",
            ((duration / 1000) / 60) / 60,
            ((duration / 1000) / 60) % 60,
            (duration / 1000) % 60
        )
    } else {
        format!(
            "{:0>2}:{:0>2}",
            ((duration / 1000) / 60) % 60,
            (duration / 1000) % 60
        )
    }
}



fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
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



    let mut r#macro: Macro = {
        if song.sheet.is_some() {
            sheet_to_macro(song.sheet.unwrap())
        } else if song.r#macro.is_some() {
            Macro::from_json(song.r#macro.unwrap())
        } else {
            println!("Could not parse, No song data.");
            return ExitCode::FAILURE;
        }
    };



    println!();
    println!("NUMPAD0 = Exit program.");
    println!("NUMPAD1 = Play.");
    println!("NUMPAD2 = Pause.");
    println!("NUMPAD3 = Stop.");
    println!();






    'mainloop: loop {
        if Keyboard::is_pressed(&Keyboard::Numpad0) {
            println!("Exiting.");
            break 'mainloop;
        }

        if Keyboard::is_pressed(&Keyboard::Numpad3) && r#macro.started() {
            println!("Reset.");
            r#macro.reset();
        }

        if Keyboard::is_pressed(&Keyboard::Numpad1) {

            println!("Playing.");

            // Wait until Numpad1 is not pressed.
            // Sometimes when its pressed and the song starts it stays down.
            while Keyboard::is_pressed(&Keyboard::Numpad1) {
                std::thread::sleep(Duration::from_millis(10));
            }

            let bar = ProgressBar::new(r#macro.total_time);
            bar.set_style(ProgressStyle::with_template("Progress {msg} [{bar:40.cyan/blue}] {percent}%")
                .unwrap()
                .progress_chars("=>-"));

            while !r#macro.finished() {

                if Keyboard::is_pressed(&Keyboard::Numpad0) {
                    println!("Exiting.");
                    break 'mainloop;
                }

                if Keyboard::is_pressed(&Keyboard::Numpad2) {
                    bar.finish_and_clear();
                    println!("Paused.");
                    continue 'mainloop;
                }

                if Keyboard::is_pressed(&Keyboard::Numpad3) {
                    bar.finish_and_clear();
                    println!("Stopped.");
                    r#macro.reset();
                    continue 'mainloop;
                }

                while !r#macro.tick_finished() {
                    r#macro.tick();
                }

                r#macro.merger.press_keys(r#macro.key_time);

                bar.set_position(r#macro.current_time);
                bar.set_message(format!(
                    "[{}]/[{}]",
                    format_duration(r#macro.current_time),
                    format_duration(r#macro.total_time)
                ));

                std::thread::sleep(Duration::from_millis(10));

            }

            r#macro.reset();

            bar.set_style(ProgressStyle::with_template("Finished {msg} [{bar:40.green/lime}] 100%")
                .unwrap()
                .progress_chars("=>-"));
            bar.finish();

        }

        std::thread::sleep(Duration::from_millis(10));
        
    }



    return ExitCode::SUCCESS;
}
