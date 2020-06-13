fn main() {
    count_chars("abradacabra");
}

fn count_chars(text: &str) -> Vec<(char, u8)> {
    let mut letters = vec![];
    let mut repetitions = vec![];
    for c in text.chars() {
        if !letters.contains(&c) {
            letters.push(c);
            repetitions.push((c, check_count_element(&c, &text)))
        }
    }
    repetitions
}

fn check_count_element(pattern: &char, target: &str) -> u8 {
    let mut count = 0;
    for letter in target.chars() {
        if letter == *pattern {
            count += 1;
        }
    }
    count
}

#[test]
fn test_count() {
    assert_eq!(
        vec![('a', 5), ('b', 2), ('r', 2), ('d', 1), ('c', 1)],
        count_chars("abradacabra")
    )
}
