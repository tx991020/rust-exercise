fn main() {
    println!("Hello, world!");
    let x1: Vec<_> = [1, 2, 3, 2, 4, 5, 4, 3]
        .iter()
        .filter_map(|x| Some(x % 2 == 0))
        .collect();
    println!("11{:?}", x1);
}
