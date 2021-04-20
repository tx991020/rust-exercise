// #![feature(core_intrinsics)]

use std::collections::HashMap;
use std::thread;

fn main() {
    // let mut x = HashMap::new();
    // x.insert("&a", 1);
    // println!("{}", type_of(&x));

    let mut v = vec![1, 7, 5, 2, 3];
    v.sort_by(|a, b| a.cmp(b));
    dbg!("{:#?}", v);

    let threads: Vec<_> = (0..100).map(|i| thread::spawn(move || f(i))).collect();

    for t in threads {
        t.join();
    }
}

// fn type_of<T>(_: &T) -> &'static str {
//     unsafe { std::intrinsics::type_name::<T>() }
// }

fn f(i: u32) {
    println!("{}", i);
}
