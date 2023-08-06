
use midi_reader_writer::{ConvertTicksToMicroseconds, midly_0_5::merge_tracks};
use midly::{Smf, live::LiveEvent, MidiMessage};

use crate::{r#macro::{Macro, EventType}, paino::Key};



pub fn midi_to_macro(midi: Smf<'_>) -> Option<Macro> {

    let mut ticks_to_microseconds = ConvertTicksToMicroseconds::try_from(midi.header).unwrap();
    

    let mut r#macro = Macro::new();

    let mut last_delay: u64 = 0;

    for (ticks, _track_index, event) in merge_tracks(&midi.tracks) {

        let ms = ticks_to_microseconds.convert(ticks, &event) / 1000;
        r#macro.add_delay((ms - last_delay) as i64);
        last_delay = ms;

        let live_event = event.as_live_event();

        if live_event.is_some() {

            let live_event = live_event.unwrap();

            match live_event {
                LiveEvent::Midi { channel: _, message } => match message {
                    MidiMessage::NoteOn { key, vel } => {
                        if vel > 0 {
                            r#macro.add_key(Key::new(key.as_int()), EventType::Key);
                        }
                    }
                    _ => {}
                }
                _ => {}
            }

        }

    }



    return Some(r#macro);

}


