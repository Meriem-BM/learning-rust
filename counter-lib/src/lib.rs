pub mod counter {
    #[derive(Default, Debug)]
    pub struct Counter {
        value: i32,
    }

    impl Counter {
        pub fn new() -> Self {
            Self { value: 0 }
        }

        pub fn increment(&mut self) {
            self.value += 1;
        }

        pub fn get(&self) -> i32 {
            self.value
        }

        #[cfg(test)]
        pub(crate) fn with_value(val: i32) -> Self {
            Self { value: val }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_increment_once() {
            let mut counter = Counter::new();
            counter.increment();
            assert_eq!(counter.get(), 1);
        }

        #[test]
        #[ignore] // intentionally ignored
        fn test_ignored_example() {
            let mut counter = Counter::new();
            counter.increment();
            counter.increment();
            assert_eq!(counter.get(), 2);
        }
    }
}
