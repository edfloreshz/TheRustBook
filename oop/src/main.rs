mod blog_types;
mod blog;
mod gui;
use gui::{Screen, Button};

fn main() {
  gui::construct();
  blog::construct();
  blog_types::construct();
}
