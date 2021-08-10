use crate::box_::List::{Cons, Nil};
use std::ops::Deref;

pub(crate) fn main() {
    using_box();

    create_list();
    deref();
}

fn create_list() {
    let list = Cons(1, Box::from(Cons(2, Box::from(Cons(3, Box::new(Nil))))));
    println!("list is {:?}", list);
}

fn using_box() {
    let b = Box::new(5);
    println!("b = {}", b);
}

// 再起エラー
// enum List {
//     Cons(i32, List),
//     Nil,
// }
#[derive(Debug)]
enum List {
    // Boxはポインターなので、再帰エラーは発生しない
    Cons(i32, Box<List>),
    Nil,
}

// Deref

fn deref() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    // = assert_eq!(5, *(y.deref()));
}

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
