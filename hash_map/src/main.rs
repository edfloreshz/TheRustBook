mod grades;
mod pig_latin;
use grades::test_grades;
use pig_latin::transform;

fn main() {
    test_grades();
    let transformed = transform("hello there".to_string());
    println!("{}", transformed);
}

