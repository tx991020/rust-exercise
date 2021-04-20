fn main() {
    let arr = [1, 2, 3, 4, 5];
    for x in arr.iter() {
        println!("{}", x);
    }
    println!("{}", "arr");
    let mut arr1 = [1, 2, 3, 4, 5];
    for x in arr1.iter_mut() {
        *x += 1;
    }
    println!("{:?}", arr1);

    let x2: Vec<_> = arr.iter().map(|x| x + 1).collect();
    println!("{:?}", x2);
    let x1 = arr.iter().any(|&x| x == 2);

    println!("{}", "x1");

    let a = [0, 1, 2];

    let mut v = vec![1, 2, 3, 4];
    v.reverse();

    println!("111{:?}", v);

    let mut v = ["a", "b", "c", "d"];
    v.swap(1, 3);
}
