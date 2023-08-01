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
    events: Vec<Event>,
    index: usize,
    next_time: u128,
    pub key_time: u64,
    pub merger: KeyMerger,
    pub current_time: u64,
    pub total_time: u64,
}

impl Macro {

    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            index: 0,
            next_time: 0,
            key_time: 15,
            merger: KeyMerger::new(),
            current_time: 0,
            total_time: 0
        }
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
        self.current_time = 0;
    }

    pub fn started(&mut self) -> bool {
        self.index > 0
    }


    pub fn add_delay(&mut self, delay: i64) {

        if delay <= 0 {
            return;
        }
        self.total_time += delay as u64;


        let last = self.events.last();

        if last.is_some() {
            let last = last.unwrap();

            match last.r#type {
                EventType::Delay => {
                    // TODO: Don't do this.
                    let mut pop = self.events.pop().unwrap();
                    pop.value += delay;
                    self.events.push(pop);
                    return;
                }
                _ => {}
            }
            
        }


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
                let key_len = self.key_time * u64::from(self.merger.sleeps());
                let delay_len = event.value as u64;
                self.next_time = get_cur_time() + u128::from(delay_len) - u128::from(key_len);
                self.current_time += delay_len;
            }
            EventType::Key => {
                self.merger.add_key(Key::new(u8::from(event.value as u8)));
            }
        }

        self.index += 1;
        
    }

}

