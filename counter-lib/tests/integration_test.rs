mod common;
use common::{CounterExt, SHARED_COUNTER};
use counter_lib::counter::Counter;

#[test]
fn test_starting_value() {
    let counter = Counter::new();
    assert_eq!(counter.get(), 0);
}

#[test]
fn test_shared_counter_increment() {
    let mut counter = SHARED_COUNTER.lock().unwrap();
    counter.reset();
    counter.increment();
    assert_eq!(counter.get(), 1);
}
