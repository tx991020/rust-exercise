use std::collections::{BTreeMap, HashMap};

fn main() {
    println!("Hello, world!");
    let mut map = HashMap::new();
    map.insert("a", 1);

    for i in 0..10 {
        println!("{}", "");
    }
    let mut map1 = BTreeMap::new();
    map1.insert("b", 2);
    map1.insert("a", 1);
    map1.insert("c", 3);

    // let l = vec![1, 2, 3];
    // let x1: Vec<_> = l.into_iter().filter(|&x| *x > 2).rev().collect();
    // println!("{:?}", x1)

    let a = vec![0, 1, 2];

    let v: Vec<_> = a.iter().filter(|&x| *x > 0).collect();
    println!("{:?}", v);

    let v1 = vec![1, 2, 3, 4, 5];
    let v_take: Vec<_> = v1.iter().take(2).collect();
    println!("{:?}", v_take)
}
