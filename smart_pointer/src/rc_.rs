use crate::rc_::List::{Cons, Nil};
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}
pub(crate) fn main() {
    let a = Rc::new(Cons(5, Rc::from(Cons(10, Rc::from(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    // cloneは参照の数を増やすだけでdeep copyは発生させない
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!(
        "count after c goes out of scope  = {}",
        Rc::strong_count(&a)
    );
    // count after creating a = 1
    // count after creating b = 2
    // count after creating c = 3
    // count after c goes out of scope  = 2
}
