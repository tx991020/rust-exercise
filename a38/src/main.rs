// 3.

#![feature(track_caller)]
use std::env::args;

use std::panic;

#[track_caller] // <-- Just add this!
pub fn my_unwrap<T>(input: Option<T>) -> T {
    match input {
        Some(t) => t,
        None => panic!("nothing to see here, move along"),
    }
}
fn main() {
    println!("args[1] = {}", my_unwrap(args().nth(1)));
    println!("args[2] = {}", my_unwrap(args().nth(2)));
    println!("args[3] = {}", my_unwrap(args().nth(3)));
    let result = panic::catch_unwind(|| {
        panic!("oh no!");
    });
}
