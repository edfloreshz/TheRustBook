pub fn test_slices() {
    let text = String::from("hello world");
    let word = first_word(&text);
    let second_word = second_word(&text);
    println!("{},{}", word, second_word);
}

fn first_word(text: &str) -> &str {
    let bytes = text.as_bytes();
    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &text[0..i];
        }
    }
    &text[..]
}

fn second_word(text: &str) -> &str {
    let bytes = text.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &text[i..text.len()]
        }
    }
    &text[..]
}