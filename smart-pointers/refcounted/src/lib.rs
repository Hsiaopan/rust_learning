#[cfg(test)]
mod tests;

use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}
