use std::collections::HashMap;

fn main() {
    count_chars("abradacabra");
}

fn count_chars(text: &str) -> Vec<(char, u8)> {
    let mut count: Vec<(char, u8)> = Vec::new();
    // let mut hashed_grades = HashMap::new();
    // match text.chars().next() {
    //     Some(letter) => match letter {
    //         'a'..='z' => {
    //             for (letter, count) in count {
    //                 if letter == letter {}
    //             }
    //             count.push((letter, 1));
    //         }
    //         _ => println!("Character not supported"),
    //     },
    //     None => println!("End"),
    // }
    count
}

#[test]
#[should_panic]
fn test_count() {
    assert_eq!(
        vec![('r', 2), ('a', 5), ('b', 2), ('c', 1)],
        count_chars("abradacabra")
    )
}
