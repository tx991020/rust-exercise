use std::collections::HashMap;
macro_rules! map {
    ($($key:expr=>$value:expr)*)=>{{
        let mut hm = HashMap::new();
        $(hm.insert($key,$value))*;
        hm
    }};
}

fn main() {
    println!("Hello, world!");

    let user = map!(
    "name"=>"Finn"
    );
    println!("{:?}", user);
}
