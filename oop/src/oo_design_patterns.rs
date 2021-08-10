use std::borrow::{Borrow, BorrowMut};

pub(crate) fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: "".to_string(),
        }
    }
    pub fn add_text(&mut self, text: &str) {
        if self.state.as_ref().unwrap().can_add_content() {
            self.content.push_str(text);
        } else {
            println!("Your state isn't allowed to add text");
        }
    }

    pub fn content(&self) -> &str {
        // stateの中身をcopyせずに参照したいのでas_ref()で参照を取得。
        // Noneであることはないので、unwrap()してOK
        // contentの実装はtraitが持つ
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
}

trait State {
    fn can_add_content(&self) -> bool {
        false
    }
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}
struct Draft {}

impl State for Draft {
    fn can_add_content(&self) -> bool {
        true
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview { approve_count: 0 })
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {
    approve_count: u32,
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        match self.approve_count {
            0 => Box::new(PendingReview { approve_count: 1 }),
            1 => Box::new(Published {}),
            _ => self,
        }
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

#[cfg(test)]
mod tests {
    use crate::oo_design_patterns::{PendingReview, Post};

    #[test]
    fn normal_sequence() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.approve();
        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
    }

    #[test]
    fn reject_and_approve() {
        let mut post = Post {
            state: Some(Box::new(PendingReview { approve_count: 0 })),
            content: "complex article".to_string(),
        };

        post.reject();
        post.request_review();
        post.approve();
        post.approve();
        assert_eq!("complex article", post.content())
    }
}
