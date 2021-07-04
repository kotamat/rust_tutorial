use crate::interior_mutability::ListMutable::{Cons, Nil};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) fn main() {
    // compile_error();
    mutable_list();
}

// fn compile_error() {
//     let x = 5;
//     let y = &mut x;
// }

pub trait Messenger {
    fn send(&self, msg: &str);
}
pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            // borrow_mutで、mutで呼び出されていなくても安全にmutableな処理を実行できる
            self.sent_messages.borrow_mut().push(msg.to_string())
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        // borrowで、リファレンスを取得できる
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

#[derive(Debug)]
enum ListMutable {
    Cons(Rc<RefCell<i32>>, Rc<ListMutable>),
    Nil,
}
fn mutable_list() {
    // let mut value = Rc::new(RefCell::new(5));
    // let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    //
    // let b = Cons(Rc::from(RefCell::new(3)), Rc::clone(&a));
    // let c = Cons(Rc::from(RefCell::new(4)), Rc::clone(&a));
    //
    // *value.borrow_mut() += 10;
    //
    // println!("a after = {:?}", a);
    // println!("b after = {:?}", b);
    // println!("c after = {:?}", c);
}
