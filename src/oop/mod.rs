mod blog_types;
mod blog;
mod gui;
use gui::{Screen, Button};

pub fn test_oop() {
  gui::construct();
  blog::construct();
  blog_types::construct();
}
