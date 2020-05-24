//  Created by Eduardo Flores on 5/18/20.
//  edfloreshz@gmail.com

mod front_of_the_house;
use front_of_the_house::{hosting, serving};

pub fn serve() {
    serving::serve_order();
}
