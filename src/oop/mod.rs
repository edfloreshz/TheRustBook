mod blog_types;
mod blog;
mod gui;

pub fn test_oop() {
  gui::construct();
  blog::construct();
  blog_types::construct();
}
