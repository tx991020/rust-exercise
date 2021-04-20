fn main() {
    let mut v = vec![1, 2];
    let mut fib = (v[0], v[1]);
    fib = (fib.1, fib.0);
    println!("{:?}", fib);
}
