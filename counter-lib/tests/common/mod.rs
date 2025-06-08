use counter_lib::counter::Counter;
use std::sync::Mutex;

pub trait CounterExt {
    fn reset(&mut self);
}

impl CounterExt for Counter {
    fn reset(&mut self) {
        *self = Counter::new();
    }
}

lazy_static::lazy_static! {
    pub static ref SHARED_COUNTER: Mutex<Counter> = Mutex::new(Counter::new());
}
