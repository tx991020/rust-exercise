#[derive(Debug)]
struct List {
    head: Option<Box<List>>,
    num: i32,
}

impl List {
    fn new() -> List {
        List { head: None, num: 0 }
    }

    fn prepend(self, elem: i32) -> List {
        List {
            head: Some(Box::new(self)),
            num: elem,
        }
    }

    fn len(&self) -> u32 {
        match &self.head {
            Some(list) => list.len() + 1,
            None => 0,
        }
    }

    fn stringify(&self) -> String {
        match &self.head {
            Some(list) => format!("{}, {}", self.num, list.stringify()),
            None => format!("Nil"),
        }
    }
}

fn print_slist(mut l: &List) {
    print!("{}", l.num);
    loop {
        match l.head {
            Some(ref head) => {
                print!(" -> {}", head.num);
                l = &(**head);
            }
            None => {
                break;
            }
        }
    }
    println!("");
}

// pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//     let mut head = head;
//     let mut tail = None;
//     while let Some(mut n) = head.take() {
//         head = n.head;
//         n.head = tail;
//         tail = Some(n);
//     }
//     tail
// }

// fn remove(mut l: List, v: i32) -> Option<i32> {
//     let mut current = &mut l.head;
//     loop {
//         match current {
//             None => return None,
//             Some(node) => {
//                 if node.num == v {
//                     *current = node.head.take();
//                     return Some(v);
//                 } else {
//                     current = &mut node.head;
//                 }
//             }
//         }
//     }
// }

fn main() {
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
    // remove(list, 2);
    // println!("{}", list.stringify());
    let result = List::new();
    let result = reverse_list(list);
    println!("{}", result.stringify());
}
