use std::any::{Any, TypeId};

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {

    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        if self.state.as_deref().unwrap().type_id() == TypeId::of::<Draft>(){
            self.content.push_str(text);
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve().unwrap())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Option<Box<dyn State>,>;
    fn content<'a>(&self,post: &'a Post) -> &'a str {
        ""
    }
}

trait Edit {
    
}
struct Draft {}

impl Edit for Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview { calls:0 })
    }

    fn approve(self: Box<Self>) -> Option<Box<dyn State>,> {
        Some(self)
    }
}


struct PendingReview {
    calls: u32 
}

impl PendingReview {
    fn reject(self:Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(mut self:Box<Self>) -> Option<Box<dyn State>,> {
        if self.calls <2 {
            self.calls+=1;
            None
        }
        else {
            Some(Box::new(Published {}))
        }
    }

}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Option<Box<dyn State>,> {
        Some(self)
    }

    fn content<'a>(&self,post:&'a Post) -> &'a str {
        &post.content
    }
}

