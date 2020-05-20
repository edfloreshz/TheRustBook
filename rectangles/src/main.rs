struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.height > rect.height && self.width > rect.width
    }
}

fn main() {
    let x: u32 = 3;
    let y: u32 = 2;
    println!("Variables: {}", area(x, y));
    let dimensions = (3,2);
    println!("Tuples: {}", area_tuple(dimensions));
    let rect = Rectangle{width: 3, height: 2};
    println!("Structures: {}", area_rect(&rect));
    println!("Implementations: {}", rect.area());
    let rect2 = Rectangle{width: 2, height: 1};
    println!("Can hold? {}", rect2.can_hold(&rect));
}

fn area(x: u32, y: u32) -> u32 {
    x * y
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_rect(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}
