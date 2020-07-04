mod blog;
mod gui;
use gui::{Screen, Button};

fn main() {
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
