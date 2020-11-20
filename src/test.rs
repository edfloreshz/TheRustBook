fn main() {
    println!("Hello, world!");
}

fn add(x: i8, y: i8) -> i8 {
    x + y
}

fn substract(x: i8, y: i8) -> i8 {
    x - y
}

#[test]
fn check_add() {
    let sum = add(2, 2);
    assert_eq!(sum, 4, "Sum was {}, expected 4", sum)
}

#[test]
fn check_substract() {
    let substraction = substract(4,2);
    assert_eq!(substraction, 2);
}
