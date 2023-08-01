use std::time::{SystemTime, UNIX_EPOCH};

use crate::{paino::{Key, KeyMerger}, song::MacroSongData};



pub enum EventType {
    Delay = 1,
    Key = 2
}


pub struct Event {
    pub r#type: EventType,
    pub value: i64,
}

impl Event {
    pub fn new(_type: EventType, _value: i64) -> Self {
        Self { 
            r#type: _type,
            value: _value
        }
    }
}





fn get_cur_time() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
}





pub struct Macro {
    pub events: Vec<Event>,
    pub index: usize,
    next_time: u128,
    pub key_time: u64,
    pub merger: KeyMerger,
}

impl Macro {

    pub fn new() -> Self {
        Self { events: Vec::new(), index: 0, next_time: 0, key_time: 15, merger: KeyMerger::new() }
    }

    pub fn from_json(data: MacroSongData) -> Self {
        let mut r#macro = Self::new();

        for event in data.events {
            match event.r#type.as_u64().unwrap() {
                1 => {
                    r#macro.add_delay(event.value.as_i64().unwrap());
                }
                2 => {
                    r#macro.add_key(Key::new(event.value.as_i64().unwrap() as u8));
                }
                _ => {}
            }
        }

        r#macro
    }

    pub fn reset(&mut self) {
        self.index = 0;
        self.next_time = 0;
    }

    pub fn started(&mut self) -> bool {
        self.index > 0
    }


    pub fn add_delay(&mut self, delay: i64) {

        if delay <= 0 {
            return;
        }

        // let last = self.events.last();

        // if last.is_some() {
        //     let last = last.unwrap();

        //     match last.r#type {
        //         EventType::Delay => {
        //             last.value += delay;
        //             return;
        //         }
        //         _ => {}
        //     }

        // }

        self.events.push(Event::new(EventType::Delay, delay));

    }

    pub fn add_key(&mut self, key: Key) {

        self.events.push(Event::new(EventType::Key, i64::from(key.midi_value)));

    }



    pub fn finished(&mut self) -> bool {
        self.index >= self.events.len()
    }

    pub fn tick_finished(&mut self) -> bool {
        self.next_time > get_cur_time() || self.finished()
    }

    pub fn tick(&mut self) {

        let event = &self.events[self.index];

        match event.r#type {
            EventType::Delay => {
                self.next_time = get_cur_time() + u128::from(event.value as u64) - u128::from(self.key_time) * u128::from(self.merger.sleeps());
            }
            EventType::Key => {
                self.merger.add_key(Key::new(u8::from(event.value as u8)));
            }
        }

        self.index += 1;
        
    }



    pub fn estimate_time(&mut self) -> u64 {
        let mut max: u64 = 0;
        for event in self.events.iter() {
            match event.r#type {
                EventType::Delay => {
                    max += event.value as u64;
                }
                _ => {}
            }
        }
        max
    }

}

