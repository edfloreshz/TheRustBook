mod front_of_the_house;
use front_of_the_house::{hosting, serving};

pub fn serve() {
    serving::serve_order();
}