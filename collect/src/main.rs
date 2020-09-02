// fn main() {
//     let strings = vec!["tofu", "93", "18"];
//     let numbers: Vec<_> = strings
//         .into_iter()
//         .map(|s| s.parse::<i32>())
//         .filter_map(Result::ok)
//         .collect();
//     println!("Results: {:?}", numbers);
// }
//

// fn main() {
//     let strings = vec!["tofu", "93", "18"];
//     let (numbers, errors): (Vec<_>, Vec<_>) = strings
//         .into_iter()
//         .map(|s| s.parse::<i32>())
//         .partition(Result::is_ok);
//     println!("Numbers: {:?}", numbers);
//     println!("Errors: {:?}", errors);
// }

use std::collections::HashMap;

fn main() {
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);

    let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}
