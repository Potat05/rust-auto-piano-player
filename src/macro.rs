use std::time::{SystemTime, UNIX_EPOCH};

use crate::paino::Key;



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
    key_time: u64,
    keys_instant: u64,
}

impl Macro {

    pub fn new() -> Self {
        Self { events: Vec::new(), index: 0, next_time: 0, key_time: 15, keys_instant: 0 }
    }

    pub fn reset(&mut self) {
        self.index = 0;
        self.next_time = 0;
        self.keys_instant = 0;
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
        self.next_time > get_cur_time()
    }

    pub fn tick(&mut self) {

        let event = &self.events[self.index];

        match event.r#type {
            EventType::Delay => {
                let keys_time = self.key_time * self.keys_instant;
                self.next_time = get_cur_time() + u128::from(event.value as u64) - u128::from(keys_time);
                self.keys_instant = 0;
            }
            EventType::Key => {
                Key::new(u8::from(event.value as u8)).press(self.key_time);
                self.keys_instant += 1;
            }
        }

        self.index += 1;
        
    }

}

