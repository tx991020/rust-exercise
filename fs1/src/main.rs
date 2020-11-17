use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let filename = "/Users/andy/GoLang/src/mgo-boot/a23/29d.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let x1: Vec<_> = reader
        .lines()
        .enumerate()
        .map(|(i, x)| return x.unwrap())
        .collect();
    println!("{:?}", x1);

    let a = [1, 2, 3];

    let iter = a.iter();

    let sum: i32 = iter.fold(0, |acc, i| acc + i);
    println!("{}", sum);

    let x2: Vec<_> = (0..10).into_iter().collect();
    println!("{:?}", x2);
    Ok(())
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    // for (index, line) in reader.lines().enumerate() {
    //     let line = line.unwrap(); // Ignore errors.
    //                               // Show the line and its number.
    //     println!("{}. {}", index + 1, line);
    // }
}
