fn main() {
    let mut total = 0;
    // compound assignment
    // there are no increment or decrement operators in rust
    total += 10;
    total += 10;
    // conversion to usize
    println!("{}", total as usize);

    // simple example of a closure
    // they are just like callbacks in javascript
    let is_even = |x: usize| -> bool { x % 2 == 0 };
    println!("{}", is_even(2));

    // A program panics when something so messed
    // up happens there must be a bug in the
    // program itself
    // a good rule of thumb is to not panic
}
