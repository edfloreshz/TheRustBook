pub trait Draw {
  fn draw(&self);
}

pub struct Screen {
  pub components: Vec<Box<dyn Draw>>
}

impl Screen {
  pub fn run(&self) {
    for component in self.components.iter() {
      component.draw()
    }
  }
}

pub struct Button {
  pub width: u32,
  pub height: u32,
  pub label: String
}

impl Draw for Button {
  fn draw(&self) {
    println!("Button named {} has width {} and height {}", self.label, self.width, self.height)
  }
}

pub fn construct() {
  let screen = Screen {
    components: vec![
      Box::new(Button{
        width: 50,
        height: 20,
        label: String::from("Start")
      })
    ]
  };
  screen.run()
}