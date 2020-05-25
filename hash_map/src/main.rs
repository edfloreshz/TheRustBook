mod grades;
mod pig_latin;
mod employee;
use crate::grades::test_grades;
use crate::pig_latin::test_transform;
use crate::employee::test_employee;

fn main() {
    test_grades();
    test_transform();
    test_employee();
}