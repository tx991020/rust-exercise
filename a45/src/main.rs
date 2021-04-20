#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    for &v in vec.iter().rev() {
        println!("{}", &v);
        let mut node = ListNode::new(v);

        node.next = current;
        current = Some(Box::new(node));
    }
    current
}

fn print_slist(mut l: &ListNode) {
    print!("{}", l.val);
    loop {
        match l.next {
            Some(ref next) => {
                print!(" -> {}", next.val);
                l = &(**next);
            }
            None => {
                break;
            }
        }
    }
    println!("");
}

fn main() {
    let v = to_list(vec![1, 2, 3, 4, 5]);
    print_slist(&v.unwrap());
}
