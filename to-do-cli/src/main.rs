fn main() {
    fn perform_operation(should_add: bool, amount_to_add: i32, value: i32) -> i32 {
        let operation: Box<dyn Fn(i32) -> i32> = if should_add {
            Box::new(move |v| v + amount_to_add)
        } else {
            Box::new(|v| v)
        };

        operation(value)
    }

    let result = perform_operation(true, 10, 5);
    println!("Result: {result}");
}
