#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl fmt::Display for List {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

use core::fmt;

use crate::List::{Cons, Nil};

// 使用 Box<T> 给递归类型一个已知的大小
fn main() {
    let list = Cons(1, 
        Box::new(Cons(2, 
            Box::new(Cons(3, 
                Box::new(Nil))))));

    println!("{}", list);

}
