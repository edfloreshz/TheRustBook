#[allow(dead_code)]
fn add(x: i8, y: i8) -> i8 {
    x + y
}

#[allow(dead_code)]
fn subtract(x: i8, y: i8) -> i8 {
    x - y
}

#[test]
fn check_add() {
    let sum = add(2, 2);
    assert_eq!(sum, 4, "Sum was {}, expected 4", sum)
}

#[test]
fn check_subtract() {
    let subtraction = subtract(4, 2);
    assert_eq!(subtraction, 2);
}
