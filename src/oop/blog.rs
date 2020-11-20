pub struct Post {
  state: Option<Box<dyn State>>,
  content: String,
  ready: bool
}

impl Post {
  pub fn new() -> Post {
    Post {
      state: Some(Box::new(Draft {})),
      content: String::new(),
      ready: false
    }
  }
  pub fn add_text(&mut self, text: &str) {
    self.content.push_str(text);
  }
  pub fn content(&self) -> &str {
    self.state.as_ref().unwrap().content(&self)
  }
  pub fn request_review(&mut self) {
    if let Some(c) = self.state.take() {
      self.state = Some(c.request_review())
    }
  }
  pub fn approve(&mut self) {
    if let Some(c) = self.state.take() {
      match self.ready {
        true => self.state = Some(c.approve()),
        false => {
          self.ready = true;
          self.state = Some(Box::new(PendingReview {}))
        }
      }
    }
  }
}

trait State {
  fn request_review(self: Box<Self>) -> Box<dyn State>;
  fn approve(self: Box<Self>) -> Box<dyn State>;
  fn content<'a>(&self, _post: &'a Post) -> &'a str { "" }
  fn reject(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    Box::new(PendingReview {})
  }
  fn approve(self: Box<Self>) -> Box<dyn State> {
    self
  }
  fn reject(self: Box<Self>) -> Box<dyn State> { self }
}

struct PendingReview {}

impl State for PendingReview {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    self
  }
  fn approve(self: Box<Self>) -> Box<dyn State> {
      Box::new(Published {})
  }
  fn content<'a>(&self, post: &'a Post) -> &'a str {
    &post.content
  }
  fn reject(self: Box<Self>) -> Box<dyn State> { Box::new(Draft {}) }
}

struct Published {}

impl State for Published {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    self
  }
  fn approve(self: Box<Self>) -> Box<dyn State> { self }
  fn content<'a>(&self, post: &'a Post) -> &'a str {
    &post.content
  }
  fn reject(self: Box<Self>) -> Box<dyn State> { self }
}

pub fn construct() {
  let mut post = Post::new();
  post.add_text("Hello there");
  post.request_review();
  post.approve(); // Ready to approve
  post.approve(); // Ready to publish
  println!("{}", post.content())
}