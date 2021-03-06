pub fn test_structs() {
    let point = Point {
        x: 1.0,
        y: 2.0
    };

    let pointer = Point {
        x: 3.0,
        y: 6.0
    };
    println!("{}", point.distance(pointer))
}

struct Point {
    x:f64, y: f64
}

impl Point {
    fn distance(self, p: Point) -> f64 {
        (self.x-p.x).hypot(self.y-p.y)
    }
}
