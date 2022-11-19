use std::{rc::Rc, cell::RefCell};

// Rust 中的智能引用计数 Rc 和 ARC
fn main() {
    let a = Rc::new(100);
    let b = a.clone(); // 不会复制内部的数据，只是增加引用计数
    let c = a.clone();

    let d = Rc::new(200);

    if c > d {
        println!("c is greater than d")
    } else {
        println!("c is less than d");
    }

    println!("a = {}, b = {}, c = {}", a, b, c);

    // 定义一个结构体来使用 RC
    let user1 = User::new(10, String::from("roseduan"));
    let user_ptr1 = Rc::new(user1);

    println!("id = {},name = {}", user_ptr1.id, user_ptr1.name);
    
    // Rc 拿到的是不可变引用，所以这里不能对 User 结构体进行修改
    // user_ptr1.set_name(String::from("roseduan11"));

    // 想要使用可变的引用，可以使用 RefCell
    let user2 = User::new(10, String::from("roseduan"));
    let user_ptr2 = RefCell::new(user2);

    // 这里就可以修改 User 结构体了
    // 把它放到单独的代码块里，是因为这里仍然需要遵守所有权规则
    // 即不能同时拥有可变借用和不可变借用
    {
        let mut mut_user = user_ptr2.borrow_mut();
        mut_user.set_name(String::from("I am roseduan"));
    }

    println!("User after update: {:?}", user_ptr2.borrow());
}

#[derive(Debug)]
struct User {
    id: u64,
    name: String,
}

impl User {
    fn new(id: u64, name: String) -> User {
        Self {
            id, name
        }
    }    

    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}
