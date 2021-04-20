use std::collections::HashMap;
use std::mem::swap;

fn main() {
    // println!("Hello, world!");
    // let x1: Vec<_> = [1, 2, 3, 2, 4, 5, 4, 3]
    //     .iter()
    //     .filter_map(|x| Some(x % 2 == 0))
    //     .collect();
    // println!("11{:?}", x1);

    let arr = vec![1, 2];

    let mut a = arr[0];
    let mut b = arr[1];
    // let (a, b) = (b, a);
    println!("{},{}", a, b);
    swap(&mut a, &mut b);
    println!("{},{}", a, b);
}
