trait Action {
    fn act(&self);
}

struct DefaultAction {
    content: String,
}

impl Action for DefaultAction {
    fn act(&self) {
        println!("do {}", self.content);
    }
}

struct User {
    action: Box<dyn Action>,
}

impl User {
    pub fn user_act(&self) {
        self.action.act()
    }
}

fn main() {
    let action = DefaultAction {
        content: "hello Rust".to_string(),
    };
    let user = User {
        action: Box::new(action),
    };
    user.user_act();
}
